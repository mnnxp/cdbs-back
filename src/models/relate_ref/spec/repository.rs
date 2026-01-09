use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::{
    language::model::SetLangName,
    spec::model::{Spec, SpecId, SpecTranslateList},
};
use crate::models::search::order::Paginate;
use crate::schema::spec_ref::dsl as spec_ref;
use crate::schema::spec_translate_list::dsl as spec_translate_list;
use diesel::prelude::*;

impl Spec {
    /// Gets spec data by id
    pub(crate) fn get_by_id(target_spec_id: i32, conn: &mut PgConnection) -> ServiceResult<Spec> {
        spec_ref::spec_ref
            .filter(spec_ref::id.eq(target_spec_id))
            .select((
                spec_ref::id,
                spec_ref::parent_spec_id,
                spec_ref::path,
            ))
            .first::<Spec>(conn)
            .map_err(|err| {
                debug!("Failed get spec by id: {}", err);
                ServiceError::InternalServerError
            })
    }
}

impl SpecTranslateList {
    /// Returns the structure of the parent catalog element
    pub(crate) fn get_parent_by_id(
        spec_id: i32,
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<SpecTranslateList> {
        let parent_spec_id = spec_ref::spec_ref
            .filter(spec_ref::id.eq(spec_id))
            .select(spec_ref::parent_spec_id)
            .first::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get specs by parent ids: {}", err);
                ServiceError::InternalServerError
            })?;
        spec_translate_list::spec_translate_list
            .filter(
                spec_translate_list::spec_id
                    .eq(&parent_spec_id)
                    .and(spec_translate_list::lang_id.eq(set_lang_id)),
            )
            .first::<SpecTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get specs: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Gets specs list witout filter
    pub(crate) fn get(
        set_lang_id: i32,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        spec_translate_list::spec_translate_list
            .filter(spec_translate_list::lang_id.eq(set_lang_id))
            .limit(paginate.limit)
            .offset(paginate.offset)
            .load::<SpecTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get specs: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Gets specs list by ids with/witout filter
    pub(crate) fn get_by_ids(
        target_specs_ids: &[i32],
        set_lang_id: i32,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        if target_specs_ids.is_empty() {
            return SpecTranslateList::get(set_lang_id, paginate, conn)
        }

        let mut query = spec_translate_list::spec_translate_list.into_boxed();
        query = match target_specs_ids.is_empty() {
            true => query.filter(spec_translate_list::lang_id.eq(set_lang_id)),
            false if target_specs_ids.len() > 1 => {
                let order_clause = diesel::dsl::sql::<diesel::sql_types::Integer>(
                    &format!("array_position(ARRAY{:?}::integer[], spec_id)", target_specs_ids)
                );
                query.filter(spec_translate_list::lang_id.eq(set_lang_id)
                    .and(spec_translate_list::spec_id.eq_any(target_specs_ids)))
                    .order(order_clause)
            },
            false => query.filter(spec_translate_list::lang_id.eq(set_lang_id)
                .and(spec_translate_list::spec_id.eq_any(target_specs_ids))),
        };
        query
            .limit(paginate.limit)
            .offset(paginate.offset)
            .load::<SpecTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed to get specs: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Gets specs list by parent ids with/witout filter
    pub(crate) fn get_by_parent_ids(
        target_specs_ids: &[i32],
        target_specs_levels: &[i32],
        set_lang_id: i32,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        // get specs for target levels
        let target_ids = match target_specs_ids.is_empty() {
            true => spec_ref::spec_ref
                .filter(spec_ref::parent_spec_id.eq_any(target_specs_levels))
                .select(spec_ref::id)
                .limit(1000)
                .load::<i32>(conn)
                .map_err(|err| {
                    debug!("Failed get specs by parent ids: {}", err);
                    ServiceError::InternalServerError
                })?,
            false => spec_ref::spec_ref
                .filter(
                    spec_ref::parent_spec_id
                        .eq_any(target_specs_levels)
                        .and(spec_ref::id.eq_any(target_specs_ids)),
                )
                .select(spec_ref::id)
                .limit(1000)
                .load::<i32>(conn)
                .map_err(|err| {
                    debug!("Failed get specs by parent ids: {}", err);
                    ServiceError::InternalServerError
                })?,
        };
        debug!("target_ids: {:?}", target_ids);

        spec_translate_list::spec_translate_list
            .filter(
                spec_translate_list::spec_id
                    .eq_any(target_ids)
                    .and(spec_translate_list::lang_id.eq(set_lang_id)),
            )
            .limit(paginate.limit)
            .offset(paginate.offset)
            .load::<SpecTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get specs: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl SpecId {
    /// Gets list all spec ids which query text
    pub(crate) fn get_list_by_name(
        query_text: &str,
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecId>> {
        let SetLangName { lang_name } = SetLangName::get_by_id(set_lang_id);

        let query = format!("
        SELECT spec_id
        FROM spec_translate_list
        WHERE to_tsvector('{lang}', spec) @@ websearch_to_tsquery('{lang}', '{query}')
        LIMIT {limit};",
            lang=lang_name,
            query=query_text,
            limit=1000);
        debug!("SQL query: {}", query);

        diesel::sql_query(query)
            .load::<SpecId>(conn)
            .map_err(|err| {
                debug!("Failed search specs: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
