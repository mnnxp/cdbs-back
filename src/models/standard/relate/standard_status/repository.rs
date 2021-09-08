use crate::errors::ServiceResult;
use crate::models::standard::standard_status::model::StandardStatusTranslateList;
use crate::schema::standard_status_translate_list::dsl as standard_status_translate_list;
use diesel::prelude::*;

impl StandardStatusTranslateList {
    /// Get standard typeanization by id and set lang
    pub fn get_standard_status_by_id(
        target_id_standard_status: &i32,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<StandardStatusTranslateList> {
        Ok(standard_status_translate_list::standard_status_translate_list
            .filter(standard_status_translate_list::id_standard_status.eq(target_id_standard_status)
            .and(standard_status_translate_list::id_lang.eq(set_id_lang)))
            .first::<StandardStatusTranslateList>(conn)?)
    }

    /// Get list standard typeanization by vec id and set lang
    pub fn get_standard_status_by_vec_id(
        target_vec_id_standard_status: &[i32],
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<StandardStatusTranslateList>> {
        Ok(standard_status_translate_list::standard_status_translate_list
            .filter(standard_status_translate_list::id_standard_status.eq_any(target_vec_id_standard_status)
            .and(standard_status_translate_list::id_lang.eq(set_id_lang)))
            .load::<StandardStatusTranslateList>(conn)?)
    }
}
