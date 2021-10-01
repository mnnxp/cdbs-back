use crate::errors::ServiceResult;
use crate::models::company::access::util::check_company_access;
use crate::models::company::member::role::util::get_company_by_role;
use diesel::{PgConnection, prelude::*};
use uuid::Uuid;

/// Get types access IDs for target role by id
pub(crate) fn _get_access_for_role(
    logged_user_uuid: &Uuid,
    target_role_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<i32>> {
    use crate::schema::role_access::dsl::*;

    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access for company with target role
    check_company_access(
        logged_user_uuid,
        &get_company_by_role(target_role_id, conn)?,
        &need_access_level,
        conn
    )?;

    Ok(role_access
        .filter(role_id.eq(target_role_id))
        .select(type_access_id)
        .load::<i32>(conn)?)
}
