use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
// use crate::models::company::member::role::model::RoleMember;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking company owner, return bool
pub(crate) fn check_is_owner_company(
    target_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let check_owner_company = company_ref::company_ref
        .filter(
            company_ref::user_uuid
                .eq(target_user_uuid)
                .and(company_ref::uuid.eq(target_company_uuid)),
        )
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check owner company: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(check_owner_company == 1)
}

/// Checking companies for user owner,
/// return true if user owner any of companies
pub(crate) fn check_is_owner_any(
    target_user_uuid: &Uuid,
    target_companies_uuids: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let check_owner_companies = company_ref::company_ref
        .filter(
            company_ref::user_uuid
                .eq(target_user_uuid)
                .and(company_ref::uuid.eq_any(target_companies_uuids)),
        )
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check owner companies: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(check_owner_companies == 1)
}

/// Checking onwed company
/// Return error if user not owned
pub(crate) fn check_is_owner_with_err(
    target_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    match check_is_owner_company(target_user_uuid, target_company_uuid, conn)? {
        true => Ok(true),
        false => Err(get_err_msg(ErrorMessage::AccessDenied)),
    }
}

/// Checks if the required access level is available, also includes ownership verification,
/// returns access denied error if the required access is not found
pub(crate) fn check_company_access(
    target_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    required_access: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // if request to view a public company
    if required_access == 3 {
        let access_type_company = get_access_type_company(target_company_uuid, conn)?;
        // if target company is public
        if access_type_company == 3 {
            return Ok(true);
        }
    }

    // check user on owner company
    if check_is_owner_company(target_user_uuid, target_company_uuid, conn)? {
        return Ok(true);
    }

    let member_role_in_company_id = member_role_in_company(target_user_uuid, target_company_uuid, conn)?;

    let found_type_access_id: i32 = get_type_access_id(member_role_in_company_id, conn)?;

    match found_type_access_id < required_access {
        true => Ok(true),
        false => Err(get_err_msg(ErrorMessage::AccessDenied)),
    }
}

/// Get role member for select user in target company
pub(crate) fn member_role_in_company(
    target_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    use crate::schema::company_member_list::dsl::*;

    // find role_id user
    company_member_list
        .filter(company_uuid.eq(target_company_uuid))
        .filter(user_uuid.eq(target_user_uuid))
        .select(role_id)
        .first(conn)
        .map_err(|err| {
            debug!("Failed get member role data: {:?}", err);
            // ServiceError::InternalServerError
            get_err_msg(ErrorMessage::AccessDenied)
        })
}

/// Get type_access_id for target role_id
pub(crate) fn get_type_access_id(
    target_role_id: i32,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    use crate::schema::role_access::dsl::*;

    // find level access for role_id
    role_access
        .filter(role_id.eq(target_role_id))
        .select(type_access_id)
        .first(conn)
        .map_err(|err| {
            debug!("Failed get type access: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Get role IDs for desired level access
pub(crate) fn get_roles_ids_for_access(
    required_access: i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<i32>> {
    use crate::schema::role_access::dsl::*;

    role_access
        .filter(type_access_id.le(required_access)) // <-- filter access < or = required_access
        .select(role_id)
        .load(conn)
        .map_err(|err| {
            debug!("Failed get role access: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Check have user among companies employees with a suitable role
pub(crate) fn check_clerk_with_suitable_role(
    target_user_uuid: &Uuid,
    target_companies_uuids: &[Uuid],
    need_roles_ids: &[i32],
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::company_member_list::dsl::*;

    // if user owner any of companies
    if check_is_owner_any(target_user_uuid, target_companies_uuids, conn)? {
        return Ok(true);
    }

    let find_provided_role = company_member_list
        .filter(
            user_uuid.eq(target_user_uuid).and(
                company_uuid
                    .eq_any(target_companies_uuids)
                    .and(role_id.eq_any(need_roles_ids)),
            ),
        )
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed get suitable role: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(find_provided_role == 1)
}

/// Gets access type for company
pub(crate) fn get_access_type_company(
    target_company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    company_ref::company_ref
        .filter(
            company_ref::uuid
                .eq(target_company_uuid)
                .and(company_ref::is_delete.eq(false)),
        )
        .select(company_ref::type_access_id)
        .first::<i32>(conn)
        .map_err(|err| {
            debug!("Not found data: {:?}", err);
            ServiceError::InternalServerError
        })
}
