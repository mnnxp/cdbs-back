// use crate::database::{db_connection, Pool};
use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::relate_ref::param::model::{
    InsertableParamTranslateList,
    IptParamTranslateListData,
    ParamTranslateList,
    Param
};
// use actix_web::web;
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_param(
    new_param_data: IptParamTranslateListData,
    conn: &PgConnection
) -> ServiceResult<ParamTranslateList> {
    use crate::schema::param_translate_list::dsl::*;

    let flag_found_param = param_translate_list
        .filter(lang_id.eq(&new_param_data.lang_id))
        .filter(paramname.eq(&new_param_data.paramname))
        .select(param_id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_param START SEARCH ={:?}", flag_found_param);

    match flag_found_param {
        0 => {
            let new_param_id = {
                use crate::schema::param_ref::dsl::*;

                let new_param: Param = diesel::insert_into(param_ref)
                    .default_values()
                    .get_result(conn)?;

                new_param.id
            };

            let new_param_data = InsertableParamTranslateList {
                param_id: new_param_id,
                lang_id: new_param_data.lang_id,
                paramname: new_param_data.paramname,
            };
            let inserted_param_data: ParamTranslateList = diesel::insert_into(param_translate_list)
                .values(&new_param_data)
                .get_result(conn)?;
            Ok(inserted_param_data)
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This param name is already there. Id: {}", flag_found_param))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
