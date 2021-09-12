use crate::errors::ServiceResult;
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use crate::schema::actual_status_translate_list::dsl as actual_status_translate_list;
use diesel::prelude::*;

impl ActualStatusTranslateList {
    /// Get actual status by id and set lang
    pub fn get_actual_status_by_id(
        target_actual_status_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ActualStatusTranslateList> {
        Ok(actual_status_translate_list::actual_status_translate_list
            .filter(actual_status_translate_list::actual_status_id.eq(target_actual_status_id)
            .and(actual_status_translate_list::lang_id.eq(set_lang_id)))
            .first::<ActualStatusTranslateList>(conn)?)
    }

    /// Get list actual status by vec id and set lang
    pub fn get_actual_status_by_vec_id(
        target_vec_actual_status_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ActualStatusTranslateList>> {
        Ok(actual_status_translate_list::actual_status_translate_list
            .filter(actual_status_translate_list::actual_status_id.eq_any(target_vec_actual_status_id)
            .and(actual_status_translate_list::lang_id.eq(set_lang_id)))
            .load::<ActualStatusTranslateList>(conn)?)
    }
}
