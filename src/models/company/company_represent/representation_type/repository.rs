use crate::errors::{ServiceResult, ServiceError};
use super::model::RepresentationTypeTranslateList;
use crate::schema::representation_type_translate_list::dsl as rttl;
use diesel::prelude::*;

impl RepresentationTypeTranslateList {
    /// Get represent type with translate by id
    pub(crate) fn get_by_id(
        target_id: &i32,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<RepresentationTypeTranslateList> {
        let result = rttl::representation_type_translate_list
            .filter(rttl::representation_type_id.eq(target_id)
            .and(rttl::lang_id.eq(set_lang_id)))
            .first::<RepresentationTypeTranslateList>(conn);

        match result {
            Ok(res) => Ok(res),
            Err(err) => {
                debug!("Not found set lang for represent: {:?}", err);
                rttl::representation_type_translate_list
                    .filter(rttl::representation_type_id.eq(target_id))
                    .first::<RepresentationTypeTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get represent types: {:?}", err);
                        ServiceError::InternalServerError
                    })
            },
        }
    }

    /// Get all represent types with translate
    pub(crate) fn get_all(
        set_lang_id: &i32,
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
