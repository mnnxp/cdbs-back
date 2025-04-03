use crate::errors::ServiceResult;
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::supplier_service::param::model::DelServiceParamData;
use crate::models::supplier_service::service::update::change_updated_at;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет параметры компонента.
/// Возвращает количество успешно удаленных параметров.
pub(crate) fn del_service_params(
    data: &DelServiceParamData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<usize> {

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::supplier_service::access::util::check_access_service_for_user(
        logged_user_uuid,
        &data.service_uuid,
        &need_access_level,
        conn
    )?;

    if data.param_ids.is_empty() {
        return Err(get_err_msg(ErrorMessage::NotFoundParamsForDeleting))
    }

    // select parameters to be delete
    let mut del_params: Vec<i32> = Vec::new();
    for pm_id in &data.param_ids {
        if pm_id > &0 { // <-- additionally we check the correctness of the id
            del_params.push(*pm_id)
        }
    }
    if del_params.is_empty() {
        return Ok(0)
    }

    // delete selected params by ids
    match delete_service_params_values(
        &data.service_uuid,
        &del_params,
        conn
    ) {
        x if x > 0 => {
            change_updated_at(&data.service_uuid, conn)?;
            Ok(x)
        },
        _ => Err(get_err_msg(ErrorMessage::CannotDeleteRows)),
    }
}

/// Delete params for target service by ids
fn delete_service_params_values(
    target_service_uuid: &Uuid,
    param_ids: &[i32],
    conn: &mut PgConnection
) -> usize {
    use crate::schema::param_to_service::dsl::*;
    let del_params = diesel::delete(param_to_service
        .filter(service_uuid.eq(target_service_uuid)
        .and(param_id.eq_any(param_ids))))
        .execute(conn);

    match del_params {
        Ok(count) => {
            debug!("Deleted {:?} rows", count);
            count   // <-- return count
        },
        Err(err) => {
            debug!("Fail delete rows:  {:?}", err);
            0       // <-- return 0
        },
    }
}
