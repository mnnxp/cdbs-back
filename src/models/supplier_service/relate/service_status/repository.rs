use crate::errors::{ServiceError, ServiceResult};
use crate::models::supplier_service::service_status::model::ServiceStatusTranslateList;
use crate::schema::service_status_translate_list::dsl as service_status_translate_list;
use diesel::prelude::*;

impl ServiceStatusTranslateList {
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
