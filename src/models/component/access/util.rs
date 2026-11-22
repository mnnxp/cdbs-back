use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::{
    check_clerk_with_suitable_role, get_roles_ids_for_access,
};
use crate::schema::company_access_to_component::dsl as company_access_to_component;
use crate::schema::company_ref::dsl as company_ref;
use crate::schema::component_ref::dsl as component_ref;
use crate::schema::user_access_to_component::dsl as user_access_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking component owner, return bool
pub(crate) fn check_is_owner(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let check_owner = component_ref::component_ref
        .filter(component_ref::user_uuid.eq(target_user_uuid))
        .filter(component_ref::uuid.eq(target_component_uuid))
        .filter(component_ref::is_delete.eq(false))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check owner component: {:?}", err);
            ServiceError::InternalServerError
        })?;
    Ok(check_owner == 1)
}

/// Checking onwed component
/// Return error if user not owned
pub(crate) fn check_is_owner_with_err(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    match check_is_owner(target_user_uuid, target_component_uuid, conn)? {
        true => Ok(true),
        false => Err(get_err_msg(ErrorMessage::AccessDenied)),
    }
}

/// Full find and check access to component for user
pub(crate) fn check_access_component_for_user(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // if request to view a public component
    if need_access_level == 3 && get_access_type_component(target_component_uuid, conn)? == 3 {
        return Ok(true);
    }
    // ownership check for ownership_check is true
    if check_is_owner(target_user_uuid, target_component_uuid, conn)? {
        return Ok(true);
    }
    if check_user_access_to_component(
        target_user_uuid,
        target_component_uuid,
        need_access_level,
        conn,
    )? {
        return Ok(true);
    }
    match check_user_access_provided_by_company(
        target_user_uuid,
        target_component_uuid,
        need_access_level,
        conn,
    )? {
        true => Ok(true),
        false => Err(get_err_msg(ErrorMessage::AccessDenied)),
    }
}

/// Checking the required level of user access to the component
pub(crate) fn check_user_access_to_component(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let check_res = user_access_to_component::user_access_to_component
        .filter(
            user_access_to_component::component_uuid
                .eq(target_component_uuid)
                .and(user_access_to_component::user_uuid.eq(target_user_uuid))
                .and(user_access_to_component::is_enabled.eq(true)),
        )
        .select(user_access_to_component::type_access_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check access component for user: {:?}", err);
            ServiceError::InternalServerError
        })?;
    Ok(matches!(check_res.first(), Some(x) if need_access_level >= *x))
}

/// Checking the availability of user access provided by the company
pub(crate) fn check_user_access_provided_by_company(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let target_companis_uuids =
        get_companies_have_access_to_component(target_component_uuid, need_access_level, conn)?;
    check_clerk_with_suitable_role(
        target_user_uuid,
        &target_companis_uuids,
        &get_roles_ids_for_access(need_access_level, conn)?,
        conn,
    )
}

/// Gets list of companies that have need level access to a component
pub(crate) fn get_companies_have_access_to_component(
    target_component_uuid: &Uuid,
    need_access_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let companies_uuids = company_access_to_component::company_access_to_component
        .filter(
            company_access_to_component::component_uuid
                .eq(target_component_uuid)
                .and(company_access_to_component::type_access_id.le(need_access_level))
                .and(company_access_to_component::is_enabled.eq(true)),
        )
        .filter(
            company_access_to_component::company_uuid.eq_any(
                company_ref::company_ref
                    .filter(company_ref::is_enabled.eq(true))
                    .filter(company_ref::is_delete.eq(false))
                    .select(company_ref::uuid),
            ),
        )
        .select(company_access_to_component::company_uuid)
        .load(conn)
        .map_err(|err| {
            debug!("Failed get companies uuids for check: {:?}", err);
            ServiceError::InternalServerError
        })?;
    match companies_uuids.is_empty() {
        true => Err(get_err_msg(ErrorMessage::AccessDenied)),
        false => Ok(companies_uuids),
    }
}

/// Gets access type for component
pub(crate) fn get_access_type_component(
    target_component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    component_ref::component_ref
        .filter(component_ref::uuid.eq(target_component_uuid))
        .filter(component_ref::is_delete.eq(false))
        .select(component_ref::type_access_id)
        .first::<i32>(conn)
        .map_err(|err| {
            debug!("Not found data: {:?}", err);
            ServiceError::InternalServerError
        })
}
