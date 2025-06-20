use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::param::model::{
    InsertableParamTranslateList, IptParamTranslateListData, ParamTranslateList,
};
use crate::schema::param_ref::dsl as param_ref;
use crate::schema::param_translate_list::dsl as param_translate_list;
use diesel::prelude::*;

/// Возвращает ParamTranslateList нового или уже существующего параметра,
/// если запрашиваемый прараметр существует, то новый не создаётся
pub(crate) fn create_param(
    new_param_data: &IptParamTranslateListData,
    conn: &mut PgConnection,
) -> ServiceResult<ParamTranslateList> {
    let flag_found = param_translate_list::param_translate_list
        .filter(
            param_translate_list::lang_id
                .eq(&new_param_data.lang_id)
                .and(param_translate_list::paramname.eq(&new_param_data.paramname)),
        )
        .limit(1)
        .load::<ParamTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed check param: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match flag_found.into_iter().next() {
        Some(x) => Ok(x), // <-- return found ParamTranslateList if name already has
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
                .returning((
                    param_translate_list::param_id,
                    param_translate_list::lang_id,
                    param_translate_list::paramname,
                ))
                .get_result::<ParamTranslateList>(conn)
                .map_err(|err| {
                    debug!("Failed insert param: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
    }
}

/// Возвращает массив с данными новых или уже существующих параметров,
/// если запрашиваемый прараметр существует, то новый не создаётся
pub(crate) fn create_parameters(
    new_params: &[IptParamTranslateListData],
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    let mut res = Vec::new();
    for np_data in new_params {
        res.push(create_param(np_data, conn)?)
    }
    Ok(res)
}
