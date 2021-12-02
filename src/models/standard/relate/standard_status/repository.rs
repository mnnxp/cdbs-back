use crate::errors::{ServiceResult, ServiceError};
use crate::models::standard::standard_status::model::StandardStatusTranslateList;
use crate::schema::standard_status_translate_list::dsl as standard_status_translate_list;
use diesel::prelude::*;

impl StandardStatusTranslateList {
    /// Get standard status by id and set lang
    pub(crate) fn get_by_id(
        target_standard_status_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<StandardStatusTranslateList> {
        let standard_status = standard_status_translate_list::standard_status_translate_list
            .filter(standard_status_translate_list::standard_status_id.eq(target_standard_status_id)
            .and(standard_status_translate_list::lang_id.eq(set_lang_id)))
            .first::<StandardStatusTranslateList>(conn);

        // if not found data for set lang
        match standard_status {
            Ok(sd_status) => Ok(sd_status),
            Err(err) => {
                debug!("Not found set lang for standard status: {:?}", err);
                standard_status_translate_list::standard_status_translate_list
                    .filter(standard_status_translate_list::standard_status_id.eq(target_standard_status_id))
                    .first::<StandardStatusTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get standard status: {:?}", err);
                        ServiceError::InternalServerError
                    })
            },
        }
    }

    /// Get standard statuses by ids and set lang
    /// if filter empty return all statuses
    pub(crate) fn get_by_ids(
        filter: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<StandardStatusTranslateList>> {
        let res = match filter.is_empty() {
            true => standard_status_translate_list::standard_status_translate_list
                .filter(standard_status_translate_list::lang_id.eq(set_lang_id))
                .load::<StandardStatusTranslateList>(conn),
            false => standard_status_translate_list::standard_status_translate_list
                .filter(standard_status_translate_list::standard_status_id.eq_any(filter)
                .and(standard_status_translate_list::lang_id.eq(set_lang_id)))
                .load::<StandardStatusTranslateList>(conn),
        };

        res.map_err(|err| {
            debug!("Failed get standard statuses: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
