use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::company::access::role_access::model::{
    InsertableRoleAccess, IptRoleAccessData, RoleAccess,
};
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::company::member::role::util::get_company_by_role;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет права доступа к роли члена компании.
pub(crate) fn create_role_access(
    logged_user_uuid: &Uuid,
    data: &IptRoleAccessData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::role_access::dsl::*;

    // check company ownership user with target role
    check_is_owner_with_err(
        logged_user_uuid,
        &get_company_by_role(data.role_id, conn)?,
        conn,
    )?;

    // check duplicate
    match role_access
        .filter(
            role_id
                .eq(&data.role_id)
                .and(type_access_id.eq_any(&data.types_access_ids)),
        )
        .execute(conn)
    {
        Ok(0) => (), // <-- not found duplicate access for role
        Ok(x) => {
            debug!("Duplicate data found: {:?}", x);
            return Err(get_err_msg(ErrorMessage::FoundDuplicateData));
        }
        Err(err) => {
            debug!("Failed check data: {:?}", err);
            return Err(get_err_msg(ErrorMessage::FailedCheckData));
        }
    }

    let insert_data: Vec<InsertableRoleAccess> = data.into();

    let res = diesel::insert_into(role_access)
        .values(insert_data)
        .get_result::<RoleAccess>(conn);

    match res {
        Ok(res_d) => {
            debug!("Add new access for role: {:?}", res_d);
            Ok(true)
        }
        Err(err) => {
            debug!("Access not added: {:?}", err);
            Err(get_err_msg(ErrorMessage::AccessNotAdded))
        }
    }

    // debug!("fn create_role_access START SEARCH ={:?}", flag_found_role_access);
}
