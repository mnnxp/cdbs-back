use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::param::model::ParamTranslateList;
use crate::schema::param_translate_list::dsl as param_translate_list;
use diesel::prelude::*;

impl ParamTranslateList {
    pub(crate) fn get_by_ids(
        target_vec_param_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        let params = param_translate_list::param_translate_list
            .filter(param_translate_list::param_id.eq_any(target_vec_param_id)
            .and(param_translate_list::lang_id.eq(set_lang_id)))
            .load::<ParamTranslateList>(conn);

        // if not found data for set lang
        match params {
            Ok(pms) => Ok(pms),
            Err(err) => {
                debug!("Not found set lang for params: {:?}", err);
                param_translate_list::param_translate_list
                    .filter(param_translate_list::param_id.eq_any(target_vec_param_id))
                    .load::<ParamTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get params: {:?}", err);
                        ServiceError::InternalServerError
                    })
            },
        }
    }
}
