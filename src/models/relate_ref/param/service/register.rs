use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::param::model::{
    InsertableParamTranslateList, IptParamTranslateListData, ParamTranslateList
};
use crate::schema::param_ref::dsl as param_ref;
use crate::schema::param_translate_list::dsl as param_translate_list;
use diesel::prelude::*;

pub(crate) fn create_param(
    new_param_data: &IptParamTranslateListData,
    conn: &PgConnection
) -> ServiceResult<ParamTranslateList> {
    let flag_found = param_translate_list::param_translate_list
        .filter(param_translate_list::lang_id.eq(&new_param_data.lang_id)
        .and(param_translate_list::paramname.eq(&new_param_data.paramname)))
        .select(param_translate_list::param_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check param: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match flag_found.first() {
        Some(x) => {
            Err(ServiceError::BadRequest(format!("This param name is already there. Id: {}", x)))
        },
        None => {
            let new_param_id = diesel::insert_into(param_ref::param_ref)
                .default_values()
                .returning(param_ref::id)
                .get_result::<i32>(conn)
                .map_err(|err| {
                    debug!("Failed insert param: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            let new_param_data = InsertableParamTranslateList {
                param_id: new_param_id,
                lang_id: new_param_data.lang_id,
                paramname: new_param_data.paramname.clone(),
            };

            diesel::insert_into(param_translate_list::param_translate_list)
                .values(&new_param_data)
                .get_result::<ParamTranslateList>(conn)
                .map_err(|err| {
                    debug!("Failed insert param: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
