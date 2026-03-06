use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::param::model::ParamTranslateList;
use crate::schema::param_translate_list::dsl as param_translate_list;
use diesel::prelude::*;

impl ParamTranslateList {
    /// Returns ParamTranslateList with translation for a specified language or by default
    pub(crate) fn get_by_id(
        param_id: i32,
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<ParamTranslateList> {
        let param = param_translate_list::param_translate_list
            .filter(
                param_translate_list::param_id
                    .eq(param_id)
                    .and(param_translate_list::lang_id.eq(set_lang_id)),
            )
            .limit(1)
            .load::<ParamTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get modification param: {:?}", err);
                ServiceError::InternalServerError
            })?;

        match param.first() {
            Some(x) => Ok(x.clone()),
            None => {
                debug!("Not found set lang for params");
                param_translate_list::param_translate_list
                    .filter(param_translate_list::param_id.eq(param_id))
                    .first::<ParamTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get params: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
        }
    }
}
