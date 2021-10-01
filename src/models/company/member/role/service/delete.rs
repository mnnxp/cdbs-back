use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::member::role::model::DelRoleMemberData;
use crate::models::company::access::util::check_company_access;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_role_member(
    logged_user_uuid: &Uuid,
    data: &DelRoleMemberData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::role_member_translate_list::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    if !check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn,
    )? {
        // return error if user not have access level
        return Err(ServiceError::BadRequest("Access denied".to_string()))
    }

    let del_role = diesel::delete(role_member_translate_list
        .filter(role_member_id.eq(&data.role_id)))
        .execute(conn);

    // debug!("fn create_role_member START SEARCH ={:?}", flag_found_role_member);

    match del_role {
        Ok(x) => {
            debug!("Completed delete role: {:#?}", x);

            Ok(x as i32)
        },
        Err(err) => {
            debug!("Error delete role: {:#?}", err);

            Err(ServiceError::BadRequest(
                "Error delete role".to_string()
            ))
        },
    }
}
