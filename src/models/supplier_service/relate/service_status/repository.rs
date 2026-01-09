use crate::errors::{ServiceError, ServiceResult};
use crate::models::supplier_service::service_status::model::ServiceStatusTranslateList;
use crate::schema::service_status_translate_list::dsl as service_status_translate_list;
use diesel::prelude::*;

impl ServiceStatusTranslateList {
    /// Get service status by id and set lang
    pub(crate) fn get_by_id(
        target_service_status_id: i32,
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<ServiceStatusTranslateList> {
        let service_status = service_status_translate_list::service_status_translate_list
            .filter(
                service_status_translate_list::service_status_id
                    .eq(target_service_status_id)
                    .and(service_status_translate_list::lang_id.eq(set_lang_id)),
            )
            .first::<ServiceStatusTranslateList>(conn);

        // if not found data for set lang
        match service_status {
            Ok(sd_status) => Ok(sd_status),
            Err(err) => {
                debug!("Not found set lang for service status: {:?}", err);
                service_status_translate_list::service_status_translate_list
                    .filter(
                        service_status_translate_list::service_status_id
                            .eq(target_service_status_id),
                    )
                    .first::<ServiceStatusTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get service status: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
        }
    }

    /// Get service statuses by ids and set lang
    /// if filter empty return all statuses
    pub(crate) fn get_by_ids(
        filter: &[i32],
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ServiceStatusTranslateList>> {
        let res = match filter.is_empty() {
            true => service_status_translate_list::service_status_translate_list
                .filter(service_status_translate_list::lang_id.eq(set_lang_id))
                .load::<ServiceStatusTranslateList>(conn),
            false => service_status_translate_list::service_status_translate_list
                .filter(
                    service_status_translate_list::service_status_id
                        .eq_any(filter)
                        .and(service_status_translate_list::lang_id.eq(set_lang_id)),
                )
                .load::<ServiceStatusTranslateList>(conn),
        };

        res.map_err(|err| {
            debug!("Failed get service statuses: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
