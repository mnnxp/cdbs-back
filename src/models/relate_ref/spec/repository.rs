use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::spec::model::{
    Spec, SpecTranslateList,
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
        Ok(spec_ref::spec_ref
            .filter(spec_ref::id.eq(target_spec_id))
            .first::<Spec>(conn)?)
    }
}

impl SpecTranslateList {
    /// Gets specs list by ids
    /// with/witout filter
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
}
