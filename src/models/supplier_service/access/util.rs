use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::company::access::util::check_is_owner_company;
use crate::schema::service_ref::dsl as service_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking service owner, return bool
pub(crate) fn check_is_owner(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    let check_owner_service = service_ref::service_ref
        .filter(service_ref::user_uuid.eq(target_user_uuid)
        .and(service_ref::uuid.eq(target_service_uuid)))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed get owner service: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(check_owner_service == 1)
}

/// Checking that the user is the owner of the company providing the service returns bool
pub(crate) fn check_is_owner_supplier(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    conn: &mut PgConnection
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
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    match check_is_owner(target_user_uuid, target_service_uuid, conn)? {
        true => Ok(true),
        false => Err(get_err_msg(ErrorMessage::AccessDenied)),
    }
}

/// Full find and check access to service for user
/// return err if not found need access
pub(crate) fn check_access_service_for_user(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    need_access_level: &i32,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    // if request to view a public service
    if need_access_level == &3 {
        let access_type_service = get_access_type_service(target_service_uuid, conn)?;
        // if target service public
        if access_type_service == 3 {
            return Ok(true)
        }
    }

    // ownership check for service
    if check_is_owner(target_user_uuid, target_service_uuid, conn)? {
        return Ok(true)
    }

    // property inspection for supplier company
    if check_is_owner_supplier(target_user_uuid, target_service_uuid, conn)? {
        return Ok(true)
    }

    // check if the user has personal access to the service
    if check_user_access_to_service(
        target_user_uuid,
        target_service_uuid,
        need_access_level,
        conn
    )? {
        return Ok(true)
    }

    // checking the availability of user access provided by the company
    if check_user_access_provided_by_company(
        target_user_uuid,
        target_service_uuid,
        need_access_level,
        conn
    )? {
        return Ok(true)
    }

    // not found need access level for target user
    Err(get_err_msg(ErrorMessage::AccessDenied))
}

/// Checking the required level of user access to the service
pub(crate) fn check_user_access_to_service(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    need_access_level: &i32,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    use crate::schema::user_access_to_service::dsl::*;

    let result_check = user_access_to_service
        .filter(service_uuid.eq(target_service_uuid)
        .and(user_uuid.eq(target_user_uuid)))
        .select(type_access_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check user access to service: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(matches!(result_check.first(), Some(x) if need_access_level >= x))
}

/// Сhecking the availability of user access provided by the company
pub(crate) fn check_user_access_provided_by_company(
    target_user_uuid: &Uuid,
    target_service_uuid: &Uuid,
    need_access_level: &i32,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    use crate::models::company::access::util::get_roles_ids_for_access;
    use crate::models::company::access::util::check_clerk_with_suitable_role;

    let target_companis_uuids = get_companies_have_access_to_service(
        target_service_uuid,
        need_access_level,
        conn
    )?;

    check_clerk_with_suitable_role(
        target_user_uuid,
        &target_companis_uuids,
        &get_roles_ids_for_access(need_access_level, conn)?,
        conn
    )
}

/// Gets list of companies that have need level access to a service
pub(crate) fn get_companies_have_access_to_service(
    target_service_uuid: &Uuid,
    need_access_level: &i32,
    conn: &mut PgConnection
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::company_access_to_service::dsl::*;

    let companies_uuids = company_access_to_service
        .filter(service_uuid.eq(target_service_uuid)
        .and(type_access_id.le(need_access_level))) // <-- access < or = need_access_level
        .select(company_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get companies list with access to service: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match companies_uuids.is_empty() {
        // not found companies with need access
        true => Err(get_err_msg(ErrorMessage::AccessDenied)),
        false => Ok(companies_uuids),
    }
}

/// Gets access type for service
pub(crate) fn get_access_type_service(
    target_service_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<i32> {
    use crate::schema::service_ref::dsl::*;

    service_ref
        .filter(uuid.eq(target_service_uuid))
        .select(type_access_id)
        .first::<i32>(conn)
        .map_err(|err| {
            debug!("Not found data: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets service status id for target service
pub(crate) fn get_service_status(
    target_service_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<i32> {
    use crate::schema::service_ref::dsl::*;

    service_ref
        .filter(uuid.eq(target_service_uuid))
        .select(service_status_id)
        .first::<i32>(conn)
        .map_err(|err| {
            debug!("Not found data: {:?}", err);
            ServiceError::InternalServerError
        })
}
