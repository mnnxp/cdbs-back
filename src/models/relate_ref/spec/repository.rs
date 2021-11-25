use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::{
    language::model::EngLangName,
    spec::model::{Spec, SpecTranslateList, SpecId}
};
use crate::schema::spec_translate_list::dsl as spec_translate_list;
use crate::schema::spec_ref::dsl as spec_ref;
use diesel::prelude::*;

impl Spec {
    /// Gets spec data by id
    pub(crate) fn get_by_id(
        target_spec_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Spec> {
        spec_ref::spec_ref
            .filter(spec_ref::id.eq(target_spec_id))
            .first::<Spec>(conn)
            .map_err(|err| {
                debug!("Failed get spec by id: {}", err);
                ServiceError::InternalServerError
            })
    }

    /// Gets spec data by parent id
    pub(crate) fn get_by_parent_id(
        target_specs_levels: &[i32],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Spec>> {
        spec_ref::spec_ref
            .filter(spec_ref::parent_spec_id.eq_any(target_specs_levels))
            .load::<Spec>(conn)
            .map_err(|err| {
                debug!("Failed get specs by parent ids: {}", err);
                ServiceError::InternalServerError
            })
    }
}

impl SpecTranslateList {
    /// Gets specs list by ids with/witout filter
    pub(crate) fn get_by_ids(
        target_specs_ids: &[i32],
        limit: &i32,
        offset: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let mut query = spec_translate_list::spec_translate_list.into_boxed();

        query = match target_specs_ids.is_empty() {
            true => {
                query.filter(spec_translate_list::lang_id.eq(set_lang_id))
            },
            false => {
                query.filter(spec_translate_list::spec_id.eq_any(target_specs_ids)
                    .and(spec_translate_list::lang_id.eq(set_lang_id)))
            },
        };

        query
            .limit(*limit as i64)
            .offset(*offset as i64)
            .load::<SpecTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get specs: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Gets specs list by parent ids with/witout filter
    pub(crate) fn get_by_parent_ids(
        target_specs_ids: &[i32],
        target_specs_levels: &[i32],
        limit: &i32,
        offset: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        // get specs for target levels
        let specs_for_levels = Spec::get_by_parent_id(
            target_specs_levels,
            conn
        )?;

        debug!("specs_for_levels: {:?}", specs_for_levels);

        let mut target_ids: Vec<i32> = Vec::new();
        for sfl in specs_for_levels {
            target_ids.push(sfl.id);
        }

        debug!("target_ids: {:?}", target_ids);

        if !target_specs_ids.is_empty() {
            target_ids.retain(|x|
                target_specs_ids.iter().any(|e| e == x)
            );
        }

        debug!("target_ids: {:?}", target_ids);

        spec_translate_list::spec_translate_list
            .filter(spec_translate_list::spec_id.eq_any(target_ids)
            .and(spec_translate_list::lang_id.eq(set_lang_id)))
            .limit(*limit as i64)
            .offset(*offset as i64)
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
        limit: &i32,
        offset: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SpecId>> {
        let EngLangName {eng_lang_name} = EngLangName::get_by_id(set_lang_id);

        let query = format!("SELECT spec_id FROM spec_translate_list WHERE to_tsvector('{eng}', spec) @@ websearch_to_tsquery('{eng}', '{query}') LIMIT {limit} OFFSET {offset}",
            eng=eng_lang_name,
            query=query_text,
            limit=limit,
            offset=offset);
        debug!("SQL query: {}", query);

        diesel::sql_query(query).load::<SpecId>(conn)
            .map_err(|err| {
                debug!("Failed search specs: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
