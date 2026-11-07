use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::standard_status::model::StandardStatusTranslateList;
use crate::schema::standard_status_translate_list::dsl as standard_status_translate_list;
use diesel::prelude::*;

impl StandardStatusTranslateList {
    /// Get standard statuses by ids and set lang
    /// if filter empty return all statuses
    pub(crate) fn get_by_ids(
        filter: &[i32],
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<StandardStatusTranslateList>> {
        let res = match filter.is_empty() {
            true => standard_status_translate_list::standard_status_translate_list
                .filter(standard_status_translate_list::lang_id.eq(set_lang_id))
                .load::<StandardStatusTranslateList>(conn),
            false => standard_status_translate_list::standard_status_translate_list
                .filter(
                    standard_status_translate_list::standard_status_id
                        .eq_any(filter)
                        .and(standard_status_translate_list::lang_id.eq(set_lang_id)),
                )
                .load::<StandardStatusTranslateList>(conn),
        };

        res.map_err(|err| {
            debug!("Failed get standard statuses: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
