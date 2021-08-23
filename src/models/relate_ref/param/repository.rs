use crate::errors::ServiceResult;
use crate::models::relate_ref::param::model::ParamTranslateList;
use crate::schema::param_translate_list::dsl as param_translate_list;
use diesel::prelude::*;

impl ParamTranslateList {
    pub fn get_param_by_id(
        target_id_param: &i32,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ParamTranslateList> {
        Ok(param_translate_list::param_translate_list
            .filter(param_translate_list::id_param.eq(target_id_param)
            .and(param_translate_list::id_lang.eq(set_id_lang)))
            .first::<ParamTranslateList>(conn)?)
    }

    pub fn get_param_by_vec_id(
        target_vec_id_param: &[i32],
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        Ok(param_translate_list::param_translate_list
            .filter(param_translate_list::id_param.eq_any(target_vec_id_param)
            .and(param_translate_list::id_lang.eq(set_id_lang)))
            .load::<ParamTranslateList>(conn)?)
    }
}
