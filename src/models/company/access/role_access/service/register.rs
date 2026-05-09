use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::company::access::role_access::model::{
    InsertableRoleAccess, IptRoleAccessData,
};
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::company::member::role::util::get_company_by_role;
use crate::schema::role_access::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет права доступа к роли члена компании.
pub(crate) fn create_role_access(
    logged_user_uuid: &Uuid,
    data: &IptRoleAccessData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    // check company ownership user with target role
    check_is_owner_with_err(
        logged_user_uuid,
        &get_company_by_role(data.role_id, conn)?,
        conn,
    )?;

    // check duplicate
    let check = role_access
        .filter(role_id.eq(&data.role_id)
            .and(type_access_id.eq_any(&data.types_access_ids)))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check data: {:?}", err);
            get_err_msg(ErrorMessage::FailedCheckData)
        })?;

    // found duplicate access for role
    if check > 0 {
        debug!("Duplicate data found: {:?}", check);
        return Err(get_err_msg(ErrorMessage::FoundDuplicateData));
    }

    let insert_data: Vec<InsertableRoleAccess> = data.into();
    let res = diesel::insert_into(role_access)
        .values(insert_data)
        .execute(conn)
        .map_err(|err| {
            debug!("Access not added: {:?}", err);
            get_err_msg(ErrorMessage::AccessNotAdded)
        })?;

    debug!("Add new access for role: {:?}", res);
    Ok(res)
}
