use crate::errors::ServiceResult;
use crate::models::relate_ref::param::model::ParamTranslateList;
use crate::schema::param_translate_list::dsl as param_translate_list;
use diesel::prelude::*;

impl ParamTranslateList {
    pub fn get_param_by_id(
        target_param_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ParamTranslateList> {
        Ok(param_translate_list::param_translate_list
            .filter(param_translate_list::param_id.eq(target_param_id)
            .and(param_translate_list::lang_id.eq(set_lang_id)))
            .first::<ParamTranslateList>(conn)?)
    }

    pub fn get_param_by_vec_id(
        target_vec_param_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        Ok(param_translate_list::param_translate_list
            .filter(param_translate_list::param_id.eq_any(target_vec_param_id)
            .and(param_translate_list::lang_id.eq(set_lang_id)))
            .load::<ParamTranslateList>(conn)?)
    }
}
