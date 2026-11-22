use super::model::RepresentationTypeTranslateList;
use crate::errors::{ServiceError, ServiceResult};
use crate::schema::representation_type_translate_list::dsl as rttl;
use diesel::prelude::*;

impl RepresentationTypeTranslateList {
    /// Get all represent types with translate
    pub(crate) fn get_all(
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<RepresentationTypeTranslateList>> {
        rttl::representation_type_translate_list
            .filter(rttl::lang_id.eq(set_lang_id))
            .order(rttl::representation_type_id.asc())
            .load::<RepresentationTypeTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get represent types: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
