use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::company::member::role::model::DelRoleMemberData;
use crate::schema::company_member_list;
use crate::schema::role_member_list;

use diesel::prelude::*;
use uuid::Uuid;

/// Deletes a company member role if no members are currently assigned to it.
pub(crate) fn del_role_member(
    logged_user_uuid: &Uuid,
    data: &DelRoleMemberData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // Verify owner permissions
    check_is_owner_with_err(logged_user_uuid, &data.company_uuid, conn)?;

    // Check if any members are still assigned to this role
    let has_members = company_member_list::table
        .select(company_member_list::role_id)
        .filter(
            company_member_list::role_id
                .eq(&data.role_id)
                .and(company_member_list::company_uuid.eq(&data.company_uuid)),
        )
        .first::<i32>(conn)
        .optional() // Returns Ok(None) instead of Err(NotFound)
        .map_err(|e| {
            debug!("Error checking role assignment: {}", e);
            ServiceError::InternalServerError
        })?
        .is_some();

    if has_members {
        // Return a specific error if the role is in use
        return Err(get_err_msg(ErrorMessage::RoleIsInUse));
    }

    // Perform deletion
    // User can only delete roles within their own company (additional layer of security).
    let deleted_rows = diesel::delete(
        role_member_list::table
            .filter(role_member_list::id.eq(&data.role_id))
            .filter(role_member_list::company_uuid.eq(&data.company_uuid)),
    )
    .execute(conn)
    .map_err(|err| {
        debug!("Failed to delete role: {}", err);
        get_err_msg(ErrorMessage::ErrorDeleteRole)
    })?;

    // Returns true if a record was actually removed, false if not found
    Ok(deleted_rows > 0)
}
