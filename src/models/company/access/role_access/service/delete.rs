use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::role_access::model::DelRoleAccessData;
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::company::member::role::util::get_company_by_role;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет права доступа роли члена компании.
pub(crate) fn del_role_access(
    logged_user_uuid: &Uuid,
    data: &DelRoleAccessData,
    conn: &mut PgConnection
) -> ServiceResult<i32> {
    use crate::schema::role_access::dsl::*;

    // check company ownership user with target role
    check_is_owner_with_err(
        logged_user_uuid,
        &get_company_by_role(&data.role_id, conn)?,
        conn
    )?;

    let res = diesel::delete(role_access
        .filter(role_id.eq(&data.role_id)
        .and(type_access_id.eq_any(&data.types_access_ids))))
        .execute(conn);

    match res {
        Ok(count) => {
            debug!("Completed delete access for role: {:?}", count);
            
            Ok(count as i32)
        },
        Err(err) => {
            debug!("Failed: access not delete: {:?}", err);

            Err(ServiceError::BadRequest(
                "Failed: access not delete".to_string()
            ))
        },
    }

    // debug!("fn create_role_access START SEARCH ={:?}", flag_found_role_access);
}
