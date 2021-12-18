use crate::errors::{ServiceResult, ServiceError};
use crate::schema::standard_ref::dsl as standard_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking standard owner, return bool
pub fn check_is_owner(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    let check_owner_standard = standard_ref::standard_ref
        .filter(standard_ref::user_uuid.eq(target_user_uuid)
        .and(standard_ref::uuid.eq(target_standard_uuid)))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed get owner standard: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(check_owner_standard == 1)
}

/// Checking onwed standard
/// Return error if user not owned
pub fn check_is_owner_with_err(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    match check_is_owner(target_user_uuid, target_standard_uuid, conn)? {
        true => Ok(true),
        false => Err(ServiceError::BadRequest("Access denied".to_string())),
    }
}

/// Full find and check access to standard for user
/// return err if not found need access
pub(crate) fn check_access_standard_for_user(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    need_access_level: &i32,
    conn: &PgConnection
) -> ServiceResult<bool> {
    // if request to view a public standard
    if need_access_level == &3 {
        let access_type_standard = get_access_type_standard(target_standard_uuid, conn)?;
        // if target standard public
        if access_type_standard == 3 {
            return Ok(true)
        }
    }

    // ownership check for ownership_check is true
    if check_is_owner(target_user_uuid, target_standard_uuid, conn)? {
        return Ok(true)
    }

    // check if the user has personal access to the standard
    if check_user_access_to_standard(
        target_user_uuid,
        target_standard_uuid,
        need_access_level,
        conn
    )? {
        return Ok(true)
    }

    // checking the availability of user access provided by the company
    if check_user_access_provided_by_company(target_user_uuid,
        target_standard_uuid,
        need_access_level,
        conn
    )? {
        return Ok(true)
    };

    // not found need access level for target user
    Err(ServiceError::BadRequest("Access denied".to_string()))
}

/// Checking the required level of user access to the standard
pub(crate) fn check_user_access_to_standard(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    need_access_level: &i32,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::user_access_to_standard::dsl::*;

    let result_check = user_access_to_standard
        .filter(standard_uuid.eq(target_standard_uuid)
        .and(user_uuid.eq(target_user_uuid)))
        .select(type_access_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check user access to standard: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(matches!(result_check.first(), Some(x) if need_access_level >= x))
}

/// Сhecking the availability of user access provided by the company
pub(crate) fn check_user_access_provided_by_company(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    need_access_level: &i32,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::models::company::access::util::get_roles_ids_for_access;
    use crate::models::company::access::util::check_clerk_with_suitable_role;

    let target_companis_uuids = get_companies_have_access_to_standard(
        target_standard_uuid,
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

/// Gets list of companies that have need level access to a standard
pub(crate) fn get_companies_have_access_to_standard(
    target_standard_uuid: &Uuid,
    need_access_level: &i32,
    conn: &PgConnection
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::company_access_to_standard::dsl::*;

    let companies_uuids = company_access_to_standard
        .filter(standard_uuid.eq(target_standard_uuid)
        .and(type_access_id.le(need_access_level))) // <-- access < or = need_access_level
        .select(company_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get companies list with access to standard: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match companies_uuids.is_empty() {
        // not found companies with need access
        true => Err(ServiceError::BadRequest("Access denied".to_string())),
        false => Ok(companies_uuids),
    }
}

/// Gets access type for standard
pub(crate) fn get_access_type_standard(
    target_standard_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::standard_ref::dsl::*;

    standard_ref
        .filter(uuid.eq(target_standard_uuid))
        .select(type_access_id)
        .first::<i32>(conn)
        .map_err(|err| {
            debug!("Not found data: {:?}", err);
            ServiceError::InternalServerError
        })
}
