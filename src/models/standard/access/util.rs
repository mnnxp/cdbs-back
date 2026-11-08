use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::{
    check_clerk_with_suitable_role, get_roles_ids_for_access,
};
use crate::schema::company_access_to_standard::dsl as company_access_to_standard;
use crate::schema::company_ref::dsl as company_ref;
use crate::schema::standard_ref::dsl as standard_ref;
use crate::schema::user_access_to_standard::dsl as user_access_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking standard owner, return bool
pub(crate) fn check_is_owner(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let check_owner = standard_ref::standard_ref
        .filter(standard_ref::user_uuid.eq(target_user_uuid))
        .filter(standard_ref::uuid.eq(target_standard_uuid))
        .filter(standard_ref::is_delete.eq(false))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed get owner standard: {:?}", err);
            ServiceError::InternalServerError
        })?;
    Ok(check_owner == 1)
}

/// Checking onwed standard
/// Return error if user not owned
pub(crate) fn check_is_owner_with_err(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    match check_is_owner(target_user_uuid, target_standard_uuid, conn)? {
        true => Ok(true),
        false => Err(get_err_msg(ErrorMessage::AccessDenied)),
    }
}

/// Full find and check access to standard for user
pub(crate) fn check_access_standard_for_user(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    if need_access_level == 3 && get_access_type_standard(target_standard_uuid, conn)? == 3 {
        return Ok(true);
    }
    if check_is_owner(target_user_uuid, target_standard_uuid, conn)? {
        return Ok(true);
    }
    if check_user_access_to_standard(
        target_user_uuid,
        target_standard_uuid,
        need_access_level,
        conn,
    )? {
        return Ok(true);
    }
    if check_user_access_provided_by_company(
        target_user_uuid,
        target_standard_uuid,
        need_access_level,
        conn,
    )? {
        return Ok(true);
    }
    Err(get_err_msg(ErrorMessage::AccessDenied))
}

/// Checking the required level of user access to the standard
pub(crate) fn check_user_access_to_standard(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let result_check = user_access_to_standard::user_access_to_standard
        .filter(
            user_access_to_standard::standard_uuid
                .eq(target_standard_uuid)
                .and(user_access_to_standard::user_uuid.eq(target_user_uuid))
                .and(user_access_to_standard::is_enabled.eq(true)),
        )
        .select(user_access_to_standard::type_access_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check user access to standard: {:?}", err);
            ServiceError::InternalServerError
        })?;
    Ok(matches!(result_check.first(), Some(x) if need_access_level >= *x))
}

/// Checking the availability of user access provided by the company
pub(crate) fn check_user_access_provided_by_company(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let target_companis_uuids =
        get_companies_have_access_to_standard(target_standard_uuid, need_access_level, conn)?;
    check_clerk_with_suitable_role(
        target_user_uuid,
        &target_companis_uuids,
        &get_roles_ids_for_access(need_access_level, conn)?,
        conn,
    )
}

/// Gets list of companies that have need level access to a standard
pub(crate) fn get_companies_have_access_to_standard(
    target_standard_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let companies_uuids = company_access_to_standard::company_access_to_standard
        .filter(
            company_access_to_standard::standard_uuid
                .eq(target_standard_uuid)
                .and(company_access_to_standard::type_access_id.le(need_access_level))
                .and(company_access_to_standard::is_enabled.eq(true)),
        )
        .filter(
            company_access_to_standard::company_uuid.eq_any(
                company_ref::company_ref
                    .filter(company_ref::is_enabled.eq(true))
                    .filter(company_ref::is_delete.eq(false))
                    .select(company_ref::uuid),
            ),
        )
        .select(company_access_to_standard::company_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!(
                "Failed get companies list with access to standard: {:?}",
                err
            );
            ServiceError::InternalServerError
        })?;
    match companies_uuids.is_empty() {
        true => Err(get_err_msg(ErrorMessage::AccessDenied)),
        false => Ok(companies_uuids),
    }
}

/// Gets access type for standard
pub(crate) fn get_access_type_standard(
    target_standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    standard_ref::standard_ref
        .filter(standard_ref::uuid.eq(target_standard_uuid))
        .filter(standard_ref::is_delete.eq(false))
        .select(standard_ref::type_access_id)
        .first::<i32>(conn)
        .map_err(|err| {
            debug!("Not found data: {:?}", err);
            ServiceError::InternalServerError
        })
}
