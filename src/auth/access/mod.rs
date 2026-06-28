//! Unified access information for users across all object types.
//! Reuses existing permission checking functions without refactoring.

mod cache;
pub(crate) use cache::{
    check_company_access, check_component_access, check_service_access, check_standard_access,
    invalidate_access, invalidate_object_cache, invalidate_user_cache,
};

use crate::errors::ServiceResult;
use crate::models::company::access::util::{
    check_is_owner_company, get_access_type_company, member_role_in_company,
};
use crate::models::component::access::util::{
    check_is_owner as component_owner, get_access_type_component,
    get_companies_have_access_to_component,
};
use crate::models::standard::access::util::{
    check_is_owner as standard_owner, get_access_type_standard,
    get_companies_have_access_to_standard,
};
use crate::models::supplier_service::access::util::{
    check_is_owner as service_owner, get_access_type_service, get_companies_have_access_to_service,
};
use crate::schema::user_access_to_component::dsl as user_access_to_component;
use crate::schema::user_access_to_service::dsl as user_access_to_service;
use crate::schema::user_access_to_standard::dsl as user_access_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Source of access for the user
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AccessSource {
    Owner,
    DirectAccess,
    CompanyRole,
    Public,
    None,
}

/// Result of access check with level and source
#[derive(Debug, Clone)]
pub(crate) struct AccessInfo {
    pub(crate) has_access: bool,
    pub(crate) access_level: Option<i32>,
    pub(crate) source: AccessSource,
}

// ========== Component ==========

/// Check user's access to a component with source information
pub(crate) fn get_component_access(
    user_uuid: &Uuid,
    component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<AccessInfo> {
    // 1. Owner check
    if component_owner(user_uuid, component_uuid, conn)? {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(1),
            source: AccessSource::Owner,
        });
    }

    // 2. Direct access check (user_access_to_component)
    if let Some(level) = get_direct_component_access(user_uuid, component_uuid, conn)? {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(level),
            source: AccessSource::DirectAccess,
        });
    }

    // 3. Company role access check
    if has_company_component_access(user_uuid, component_uuid, conn).is_ok() {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(3),
            source: AccessSource::CompanyRole,
        });
    }

    // 4. Public access check
    if let Ok(access_type) = get_access_type_component(component_uuid, conn) {
        if access_type == 3 {
            return Ok(AccessInfo {
                has_access: true,
                access_level: Some(3),
                source: AccessSource::Public,
            });
        }
    }

    Ok(AccessInfo {
        has_access: false,
        access_level: None,
        source: AccessSource::None,
    })
}

/// Get direct access level from user_access_to_component table
fn get_direct_component_access(
    user_uuid: &Uuid,
    component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Option<i32>> {
    let level = user_access_to_component::user_access_to_component
        .filter(
            user_access_to_component::component_uuid
                .eq(component_uuid)
                .and(
                    user_access_to_component::user_uuid
                        .eq(user_uuid)
                        .and(user_access_to_component::is_enabled.eq(true)),
                ),
        )
        .select(user_access_to_component::type_access_id)
        .first(conn)
        .optional()?;

    Ok(level)
}

/// Check if user has access through company membership and roles
fn has_company_component_access(
    user_uuid: &Uuid,
    component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let companies = get_companies_have_access_to_component(component_uuid, 3, conn)?;

    for company_uuid in companies {
        if member_role_in_company(user_uuid, &company_uuid, conn).is_ok() {
            return Ok(true);
        }
    }
    Ok(false)
}

// ========== Standard ==========

/// Check user's access to a standard with source information
pub(crate) fn get_standard_access(
    user_uuid: &Uuid,
    standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<AccessInfo> {
    if standard_owner(user_uuid, standard_uuid, conn)? {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(1),
            source: AccessSource::Owner,
        });
    }

    if let Some(level) = get_direct_standard_access(user_uuid, standard_uuid, conn)? {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(level),
            source: AccessSource::DirectAccess,
        });
    }

    if has_company_standard_access(user_uuid, standard_uuid, conn).is_ok() {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(3),
            source: AccessSource::CompanyRole,
        });
    }

    if let Ok(access_type) = get_access_type_standard(standard_uuid, conn) {
        if access_type == 3 {
            return Ok(AccessInfo {
                has_access: true,
                access_level: Some(3),
                source: AccessSource::Public,
            });
        }
    }

    Ok(AccessInfo {
        has_access: false,
        access_level: None,
        source: AccessSource::None,
    })
}

fn get_direct_standard_access(
    user_uuid: &Uuid,
    standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Option<i32>> {
    let level = user_access_to_standard::user_access_to_standard
        .filter(
            user_access_to_standard::standard_uuid
                .eq(standard_uuid)
                .and(
                    user_access_to_standard::user_uuid
                        .eq(user_uuid)
                        .and(user_access_to_standard::is_enabled.eq(true)),
                ),
        )
        .select(user_access_to_standard::type_access_id)
        .first(conn)
        .optional()?;

    Ok(level)
}

fn has_company_standard_access(
    user_uuid: &Uuid,
    standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let companies = get_companies_have_access_to_standard(standard_uuid, 3, conn)?;

    for company_uuid in companies {
        if member_role_in_company(user_uuid, &company_uuid, conn).is_ok() {
            return Ok(true);
        }
    }
    Ok(false)
}

// ========== Service ==========

/// Check user's access to a service with source information
pub(crate) fn get_service_access(
    user_uuid: &Uuid,
    service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<AccessInfo> {
    if service_owner(user_uuid, service_uuid, conn)? {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(1),
            source: AccessSource::Owner,
        });
    }

    if let Some(level) = get_direct_service_access(user_uuid, service_uuid, conn)? {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(level),
            source: AccessSource::DirectAccess,
        });
    }

    if has_company_service_access(user_uuid, service_uuid, conn).is_ok() {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(3),
            source: AccessSource::CompanyRole,
        });
    }

    if let Ok(access_type) = get_access_type_service(service_uuid, conn) {
        if access_type == 3 {
            return Ok(AccessInfo {
                has_access: true,
                access_level: Some(3),
                source: AccessSource::Public,
            });
        }
    }

    Ok(AccessInfo {
        has_access: false,
        access_level: None,
        source: AccessSource::None,
    })
}

fn get_direct_service_access(
    user_uuid: &Uuid,
    service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Option<i32>> {
    let level = user_access_to_service::user_access_to_service
        .filter(
            user_access_to_service::service_uuid.eq(service_uuid).and(
                user_access_to_service::user_uuid
                    .eq(user_uuid)
                    .and(user_access_to_service::is_enabled.eq(true)),
            ),
        )
        .select(user_access_to_service::type_access_id)
        .first(conn)
        .optional()?;

    Ok(level)
}

fn has_company_service_access(
    user_uuid: &Uuid,
    service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let companies = get_companies_have_access_to_service(service_uuid, 3, conn)?;

    for company_uuid in companies {
        if member_role_in_company(user_uuid, &company_uuid, conn).is_ok() {
            return Ok(true);
        }
    }
    Ok(false)
}

// ========== Company ==========

/// Check user's access to a company with source information
pub(crate) fn get_company_access(
    user_uuid: &Uuid,
    company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<AccessInfo> {
    debug!("1. Owner check");
    // 1. Owner check
    if check_is_owner_company(user_uuid, company_uuid, conn)? {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(1),
            source: AccessSource::Owner,
        });
    }

    debug!("2. Company member with role");
    // 2. Company member with role
    if member_role_in_company(user_uuid, company_uuid, conn).is_ok() {
        return Ok(AccessInfo {
            has_access: true,
            access_level: Some(2),
            source: AccessSource::CompanyRole,
        });
    }

    debug!("3. Public access check");
    // 3. Public access check
    if let Ok(access_type) = get_access_type_company(company_uuid, conn) {
        if access_type == 3 {
            return Ok(AccessInfo {
                has_access: true,
                access_level: Some(3),
                source: AccessSource::Public,
            });
        }
    }

    Ok(AccessInfo {
        has_access: false,
        access_level: None,
        source: AccessSource::None,
    })
}
