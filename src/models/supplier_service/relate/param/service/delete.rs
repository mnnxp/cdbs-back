use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::supplier_service::access::util::check_access_service_for_user;
use crate::models::supplier_service::param::model::DelServiceParamData;
use crate::models::supplier_service::service::update::change_service_updated_at;
use crate::schema::param_to_service::dsl as param_to_service;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет параметры компонента.
/// Возвращает количество успешно удаленных параметров.
pub(crate) fn del_service_params(
    data: &DelServiceParamData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)
    check_access_service_for_user(
        logged_user_uuid,
        &data.service_uuid,
        need_access_level,
        conn,
    )?;

    if data.param_ids.is_empty() {
        return Err(get_err_msg(ErrorMessage::NotFoundParamsForDeleting));
    }

    // select parameters to be delete
    let mut del_params: Vec<i32> = Vec::new();
    for pm_id in &data.param_ids {
        if pm_id > &0 {
            // <-- additionally we check the correctness of the id
            del_params.push(*pm_id)
        }
    }
    if del_params.is_empty() {
        return Ok(0);
    }
    let old_data: Vec<(i32, String)> = param_to_service::param_to_service
        .filter(
            (param_to_service::service_uuid.eq(&data.service_uuid))
                .and(param_to_service::param_id.eq_any(&del_params)),
        )
        .select((param_to_service::param_id, param_to_service::value))
        .load::<(i32, String)>(conn)
        .map_err(|err| {
            debug!("Failed get service params: {:?}", err);
            get_err_msg(ErrorMessage::FailedCheckData)
        })?;
    // delete selected params by ids
    let res_del = delete_service_params_values(&data.service_uuid, &del_params, conn)?;
    change_service_updated_at(
        &data.service_uuid,
        logged_user_uuid,
        format!("Deleted the parameters: {old_data:?}"),
        conn,
    )?;
    Ok(res_del)
}

/// Delete params for target service by ids
fn delete_service_params_values(
    target_service_uuid: &Uuid,
    param_ids: &[i32],
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    diesel::delete(
        param_to_service::param_to_service.filter(
            param_to_service::service_uuid
                .eq(target_service_uuid)
                .and(param_to_service::param_id.eq_any(param_ids)),
        ),
    )
    .execute(conn)
    .map_err(|err| {
        debug!("Fail delete rows:  {:?}", err);
        get_err_msg(ErrorMessage::CannotDeleteRows)
    })
}
