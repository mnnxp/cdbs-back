use crate::errors::ServiceResult;
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::component::component_modification::param::model::DelModificationParamData;
use crate::models::component::component_modification::util::get_component_by_modification;
use crate::models::component::service::update::change_updated_at;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет параметры модификации компонента.
pub(crate) fn del_modification_params(
    logged_user_uuid: &Uuid,
    data: &DelModificationParamData,
    conn: &mut PgConnection
) -> ServiceResult<usize> {

    let need_access_level = 1; // todo!(create enum for manage access level)
    let target_component_uuid = get_component_by_modification(&data.modification_uuid, conn)?;
    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &target_component_uuid,
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
    match delete_modification_params_values(
        &data.modification_uuid,
        &del_params,
        conn
    ) {
        x if x > 0 => {
            change_updated_at(&target_component_uuid, Some(&data.modification_uuid), conn)?;
            Ok(x)
        },
        _ => Err(get_err_msg(ErrorMessage::CannotDeleteRows)),
    }
}

/// Delete params for target component modification by ids
fn delete_modification_params_values(
    target_modification_uuid: &Uuid,
    param_ids: &[i32],
    conn: &mut PgConnection
) -> usize {
    use crate::schema::param_to_modification::dsl::*;
    let del_params = diesel::delete(param_to_modification
        .filter(modification_uuid.eq(target_modification_uuid)
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
