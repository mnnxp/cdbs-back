use crate::errors::{ServiceResult, ServiceError};
use super::model::RepresentationTypeTranslateList;
use crate::schema::representation_type_translate_list::dsl::*;
use diesel::prelude::*;

impl RepresentationTypeTranslateList {
    /// Get represent type with translate by ids
    pub fn get_by_ids(
        target_ids: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<RepresentationTypeTranslateList>> {
        representation_type_translate_list
            .filter(representation_type_id.eq_any(target_ids)
            .and(lang_id.eq(set_lang_id)))
            .load::<RepresentationTypeTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get represent types: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Get all represent types with translate
    pub fn get_all(
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<RepresentationTypeTranslateList>> {
        representation_type_translate_list
            .filter(lang_id.eq(set_lang_id))
            .load::<RepresentationTypeTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get represent types: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
