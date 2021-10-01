use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::role_access::model::{
    RoleAccess,
    IptRoleAccessData,
    InsertableRoleAccess,
};
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::company::member::role::util::get_company_by_role;
use diesel::prelude::*;
use uuid::Uuid;

/// Add new access for role
pub(crate) fn create_role_access(
    logged_user_uuid: &Uuid,
    data: &IptRoleAccessData,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::role_access::dsl::*;

    // check company ownership user with target role
    check_is_owner_with_err(
        logged_user_uuid,
        &get_company_by_role(&data.role_id, conn)?,
        conn
    )?;

    // check duplicate
    match role_access.filter(role_id.eq(&data.role_id)
        .and(type_access_id.eq_any(&data.types_access_ids)))
        .execute(conn) {
        Ok(0) => (), // <-- not found duplicate access for role
        Ok(x) => {
            debug!("Duplicate data found: {:?}", x);

            return Err(ServiceError::BadRequest(
                "Duplicate data found".to_string()
            ))
        },
        Err(err) => {
            debug!("Failed check data: {:?}", err);

            return Err(ServiceError::BadRequest(
                "Failed check data".to_string()
            ))
        },
    }

    let insert_data: Vec<InsertableRoleAccess> = data.into();

    let res = diesel::insert_into(role_access)
        .values(insert_data)
        .get_result::<RoleAccess>(conn);

    match res {
        Ok(res_d) => {
            debug!("Add new access for role: {:?}", res_d);
            Ok(true)
        },
        Err(err) => {
            debug!("Access not added: {:?}", err);

            Err(ServiceError::BadRequest(
                "Access not added".to_string()
            ))
        },
    }

    // debug!("fn create_role_access START SEARCH ={:?}", flag_found_role_access);
}
