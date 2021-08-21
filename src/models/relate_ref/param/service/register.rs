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
        .filter(id_lang.eq(&new_param_data.id_lang))
        .filter(paramname.eq(&new_param_data.paramname))
        .select(id_param)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_param START SEARCH ={:?}", flag_found_param);

    match flag_found_param {
        0 => {
            let new_id_param = {
                use crate::schema::param_ref::dsl::*;

                let new_param: Param = diesel::insert_into(param_ref)
                    .default_values()
                    .get_result(conn)?;

                new_param.id
            };

            let new_param_data = InsertableParamTranslateList {
                id_param: new_id_param,
                id_lang: new_param_data.id_lang,
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
