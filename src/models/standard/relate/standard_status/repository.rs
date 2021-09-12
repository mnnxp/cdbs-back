use crate::errors::ServiceResult;
use crate::models::standard::standard_status::model::StandardStatusTranslateList;
use crate::schema::standard_status_translate_list::dsl as standard_status_translate_list;
use diesel::prelude::*;

impl StandardStatusTranslateList {
    /// Get standard typeanization by id and set lang
    pub fn get_standard_status_by_id(
        target_standard_status_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<StandardStatusTranslateList> {
        Ok(standard_status_translate_list::standard_status_translate_list
            .filter(standard_status_translate_list::standard_status_id.eq(target_standard_status_id)
            .and(standard_status_translate_list::lang_id.eq(set_lang_id)))
            .first::<StandardStatusTranslateList>(conn)?)
    }

    /// Get list standard typeanization by vec id and set lang
    pub fn get_standard_status_by_vec_id(
        target_vec_standard_status_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<StandardStatusTranslateList>> {
        Ok(standard_status_translate_list::standard_status_translate_list
            .filter(standard_status_translate_list::standard_status_id.eq_any(target_vec_standard_status_id)
            .and(standard_status_translate_list::lang_id.eq(set_lang_id)))
            .load::<StandardStatusTranslateList>(conn)?)
    }
}
