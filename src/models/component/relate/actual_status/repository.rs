use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use crate::schema::actual_status_translate_list::dsl as actual_status_translate_list;
use diesel::prelude::*;

impl ActualStatusTranslateList {
    /// Get actual status by id and set lang
    pub(crate) fn get_actual_status_by_id(
        target_actual_status_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ActualStatusTranslateList> {
        let actual_status = actual_status_translate_list::actual_status_translate_list
            .filter(actual_status_translate_list::actual_status_id.eq(target_actual_status_id)
            .and(actual_status_translate_list::lang_id.eq(set_lang_id)))
            .first::<ActualStatusTranslateList>(conn);

        // if not found data for set lang
        match actual_status {
            Ok(at_status) => Ok(at_status),
            Err(err) => {
                debug!("Not found set lang for actual status: {:?}", err);
                actual_status_translate_list::actual_status_translate_list
                    .filter(actual_status_translate_list::actual_status_id.eq(target_actual_status_id))
                    .first::<ActualStatusTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get actual status: {:?}", err);
                        ServiceError::InternalServerError
                    })
            },
        }
    }

    /// Get list actual status by vec id and set lang
    pub(crate) fn get_actual_status_by_vec_id(
        target_vec_actual_status_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ActualStatusTranslateList>> {
        let actuals_status = actual_status_translate_list::actual_status_translate_list
            .filter(actual_status_translate_list::actual_status_id.eq_any(target_vec_actual_status_id)
            .and(actual_status_translate_list::lang_id.eq(set_lang_id)))
            .load::<ActualStatusTranslateList>(conn);

        // if not found data for set lang
        match actuals_status {
            Ok(ats_status) => Ok(ats_status),
            Err(err) => {
                debug!("Not found set lang for actuals status: {:?}", err);
                actual_status_translate_list::actual_status_translate_list
                    .filter(actual_status_translate_list::actual_status_id.eq_any(target_vec_actual_status_id))
                    .load::<ActualStatusTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get actual status: {:?}", err);
                        ServiceError::InternalServerError
                    })
            },
        }
    }
}
