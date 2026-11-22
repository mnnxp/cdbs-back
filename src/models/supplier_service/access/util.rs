use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::{
    check_clerk_with_suitable_role, check_is_owner_company, get_roles_ids_for_access,
};
use crate::schema::company_access_to_service::dsl as company_access_to_service;
use crate::schema::company_ref::dsl as company_ref;
use crate::schema::service_ref::dsl as service_ref;
use crate::schema::user_access_to_service::dsl as user_access_to_service;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking service owner, return bool
pub(crate) fn check_is_owner(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let check_owner = service_ref::service_ref
        .filter(service_ref::user_uuid.eq(target_user_uuid))
        .filter(service_ref::uuid.eq(target_service_uuid))
        .filter(service_ref::is_delete.eq(false))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed get owner service: {:?}", err);
            ServiceError::InternalServerError
        })?;
    Ok(check_owner == 1)
}

/// Checking that the user is the owner of the company providing the service returns bool
pub(crate) fn check_is_owner_supplier(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let company_uuid = service_ref::service_ref
        .filter(service_ref::uuid.eq(target_service_uuid))
        .select(service_ref::company_uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get data of service for check: {:?}", err);
            ServiceError::InternalServerError
        })?;
    check_is_owner_company(target_user_uuid, &company_uuid, conn)
}

/// Checking onwed service
/// Return error if user not owned
pub(crate) fn check_is_owner_with_err(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    match check_is_owner(target_user_uuid, target_service_uuid, conn)? {
        true => Ok(true),
        false => Err(get_err_msg(ErrorMessage::AccessDenied)),
    }
}

/// Full find and check access to service for user
pub(crate) fn check_access_service_for_user(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    if need_access_level == 3 && get_access_type_service(target_service_uuid, conn)? == 3 {
        return Ok(true);
    }
    if check_is_owner(target_user_uuid, target_service_uuid, conn)? {
        return Ok(true);
    }
    if check_is_owner_supplier(target_user_uuid, target_service_uuid, conn)? {
        return Ok(true);
    }
    if check_user_access_to_service(
        target_user_uuid,
        target_service_uuid,
        need_access_level,
        conn,
    )? {
        return Ok(true);
    }
    if check_user_access_provided_by_company(
        target_user_uuid,
        target_service_uuid,
        need_access_level,
        conn,
    )? {
        return Ok(true);
    }
    Err(get_err_msg(ErrorMessage::AccessDenied))
}

/// Checking the required level of user access to the service
pub(crate) fn check_user_access_to_service(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let result_check = user_access_to_service::user_access_to_service
        .filter(
            user_access_to_service::service_uuid
                .eq(target_service_uuid)
                .and(user_access_to_service::user_uuid.eq(target_user_uuid))
                .and(user_access_to_service::is_enabled.eq(true)),
        )
        .select(user_access_to_service::type_access_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check user access to service: {:?}", err);
            ServiceError::InternalServerError
        })?;
    Ok(matches!(result_check.first(), Some(x) if need_access_level >= *x))
}

/// Checking the availability of user access provided by the company
pub(crate) fn check_user_access_provided_by_company(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let target_companis_uuids =
        get_companies_have_access_to_service(target_service_uuid, need_access_level, conn)?;
    check_clerk_with_suitable_role(
        target_user_uuid,
        &target_companis_uuids,
        &get_roles_ids_for_access(need_access_level, conn)?,
        conn,
    )
}

/// Gets list of companies that have need level access to a service
pub(crate) fn get_companies_have_access_to_service(
    target_service_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let companies_uuids = company_access_to_service::company_access_to_service
        .filter(
            company_access_to_service::service_uuid
                .eq(target_service_uuid)
                .and(company_access_to_service::type_access_id.le(need_access_level))
                .and(company_access_to_service::is_enabled.eq(true)),
        )
        .filter(
            company_access_to_service::company_uuid.eq_any(
                company_ref::company_ref
                    .filter(company_ref::is_enabled.eq(true))
                    .filter(company_ref::is_delete.eq(false))
                    .select(company_ref::uuid),
            ),
        )
        .select(company_access_to_service::company_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!(
                "Failed get companies list with access to service: {:?}",
                err
            );
            ServiceError::InternalServerError
        })?;
    match companies_uuids.is_empty() {
        true => Err(get_err_msg(ErrorMessage::AccessDenied)),
        false => Ok(companies_uuids),
    }
}

/// Gets access type for service
pub(crate) fn get_access_type_service(
    target_service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    service_ref::service_ref
        .filter(service_ref::uuid.eq(target_service_uuid))
        .filter(service_ref::is_delete.eq(false))
        .select(service_ref::type_access_id)
        .first::<i32>(conn)
        .map_err(|err| {
            debug!("Not found data: {:?}", err);
            ServiceError::InternalServerError
        })
}
