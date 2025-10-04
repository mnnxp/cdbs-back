use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use crate::schema::actual_status_translate_list::dsl as actual_status_translate_list;
use diesel::prelude::*;

impl ActualStatusTranslateList {
    /// Get actual status by id and set lang
    pub(crate) fn get_by_id(
        target_actual_status_id: &i32,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<ActualStatusTranslateList> {
        let actual_status = actual_status_translate_list::actual_status_translate_list
            .filter(
                actual_status_translate_list::actual_status_id
                    .eq(target_actual_status_id)
                    .and(actual_status_translate_list::lang_id.eq(set_lang_id)),
            )
            .limit(1)
            .load::<ActualStatusTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get actual status: {:?}", err);
                ServiceError::InternalServerError
            })?;

        // if not found data for set lang
        match actual_status.first() {
            Some(x) => Ok(x.clone()),
            None => {
                debug!("Not found set lang for actual status");
                actual_status_translate_list::actual_status_translate_list
                    .filter(
                        actual_status_translate_list::actual_status_id.eq(target_actual_status_id),
                    )
                    .first::<ActualStatusTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get actual status: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
        }
    }

    /// Get component actual statuses by ids and set lang
    /// if filter empty return all statuses
    pub(crate) fn get_by_ids(
        filter: &[i32],
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ActualStatusTranslateList>> {
        let res = match filter.is_empty() {
            true => actual_status_translate_list::actual_status_translate_list
                .filter(actual_status_translate_list::lang_id.eq(set_lang_id))
                .load::<ActualStatusTranslateList>(conn),
            false => actual_status_translate_list::actual_status_translate_list
                .filter(
                    actual_status_translate_list::actual_status_id
                        .eq_any(filter)
                        .and(actual_status_translate_list::lang_id.eq(set_lang_id)),
                )
                .load::<ActualStatusTranslateList>(conn),
        };

        res.map_err(|err| {
            debug!("Failed get component actual statuses: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
