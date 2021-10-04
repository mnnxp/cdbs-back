use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::member::role::model::RoleMember;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking company owner, return bool
pub fn check_is_owner(
    target_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    conn: &PgConnection
) -> bool {
    use crate::schema::company_ref::dsl::*;

    let check_owner_company = company_ref
        .filter(user_uuid.eq(target_user_uuid)
        .and(uuid.eq(target_company_uuid)))
        .limit(1)
        .execute(conn);

    match check_owner_company {
        Ok(count) if count == 1 => true,
        Ok(_) => false,
        Err(err) => {
            debug!("Failed check data: {:?}", err);
            false
        },
    }
}

/// Checking companies for user owner,
/// return true if user owner any of companies
pub fn check_is_owner_any(
    target_user_uuid: &Uuid,
    target_companies_uuids: &[Uuid],
    conn: &PgConnection
) -> bool {
    use crate::schema::company_ref::dsl::*;

    let check_owner_companies = company_ref
        .filter(user_uuid.eq(target_user_uuid)
        .and(uuid.eq_any(target_companies_uuids)))
        .limit(1)
        .execute(conn);

    match check_owner_companies {
        Ok(count) if count == 1 => true,
        Ok(_) => false,
        Err(err) => {
            debug!("Failed check data: {:?}", err);
            false
        },
    }
}

/// Checking onwed company
/// Return error if user not owned
pub fn check_is_owner_with_err(
    target_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    match check_is_owner(target_user_uuid, target_company_uuid, conn) {
        true => Ok(true),
        false => Err(ServiceError::BadRequest(
            "Access denied".to_string(),
        )),
    }
}

/// Gets list of users uuids that have need level access to a company
pub(crate) fn _get_users_have_access_to_company(
    target_company_uuid: &Uuid,
    required_access: &i32,
    conn: &PgConnection
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::company_member_list::dsl::*;

    let suitable_role = RoleMember::get_roles_for_type_access(
        required_access,
        conn
    );

    if suitable_role.is_empty() {
        return Err(ServiceError::BadRequest(
            "Not found set access for target role".to_string()
        ))
    }

    let users_uuids = company_member_list
        .filter(company_uuid.eq(target_company_uuid)
        .and(role_id.eq_any(&suitable_role)))
        .select(user_uuid)
        .load::<Uuid>(conn);

    match users_uuids {
        Ok(ur_uuids) if !ur_uuids.is_empty() => Ok(ur_uuids),
        // not found companies with need access
        Ok(_) => Err(ServiceError::BadRequest(
            "Mot found users with access target company".to_string()
        )),
        Err(err) => {
            debug!("Failed check data: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed check data".to_string()
            ))
        },
    }
}

/// Checking the availability of the required access level
/// with ownership check
pub(crate) fn check_company_access(
    target_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    required_access: &i32,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // if request to view a public company
    if required_access == &3 {
        let access_type_company = get_access_type_company(target_company_uuid, conn)?;
        // if target company is public
        if access_type_company == 3 {
            return Ok(true)
        }
    }

    // check user on owner company
    if check_is_owner(
        target_user_uuid,
        target_company_uuid,
        conn
    ) {
        return Ok(true)
    }

    let found_type_access_id: i32 = get_type_access_id(
        &member_role_in_company(
            target_user_uuid,
            target_company_uuid,
            conn,
        ),
        conn
    );

    match found_type_access_id {
        1..=i32::MAX => {
            // debug!("get_type_access_id ({:?}) < required_access ({:?})", &get_type_access_id, &required_access);
            if &found_type_access_id < required_access {
                Ok(true)
            } else {
                Err(ServiceError::BadRequest(
                    "Access denied".to_string(),
                ))
            }
        }
        _ => Err(ServiceError::BadRequest("Access denied".to_string())),
    }
}

/// Get role member for select user in target company
pub(crate) fn member_role_in_company(
    target_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    conn: &PgConnection,
) -> i32 {
    use crate::schema::company_member_list::dsl::*;

    // find role_id user
    company_member_list
        .filter(company_uuid.eq(target_company_uuid))
        .filter(user_uuid.eq(target_user_uuid))
        .select(role_id)
        .first(conn)
        .unwrap_or(0)
}

/// Search type_access_id for target role_id
pub(crate) fn get_type_access_id(
    target_role_id: &i32,
    conn: &PgConnection,
) -> i32 {
    use crate::schema::role_access::dsl::*;

    // find level access for role_id
    role_access
        .filter(role_id.eq(target_role_id))
        .select(type_access_id)
        .first(conn)
        .unwrap_or(0)
}

/// Get role IDs for desired level access
pub(crate) fn get_roles_ids_for_access(
    required_access: &i32,
    conn: &PgConnection
) -> ServiceResult<Vec<i32>> {
    use crate::schema::role_access::dsl::*;

    let roles_ids = role_access
        .filter(type_access_id.le(required_access)) // <-- filter access < or = required_access
        .select(role_id)
        .load(conn);

    match roles_ids {
        Ok(rs_ids) => Ok(rs_ids),
        Err(err) => {
            debug!("Failed get data: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed get data".to_string()
            ))
        },
    }
}

/// Check have user among companies employees with a suitable role
pub(crate) fn check_clerk_with_suitable_role(
    target_user_uuid: &Uuid,
    target_companies_uuids: &[Uuid],
    need_roles_ids: &[i32],
    conn: &PgConnection
) -> bool {
    use crate::schema::company_member_list::dsl::*;

    // if user owner any of companies
    if check_is_owner_any(target_user_uuid, target_companies_uuids, conn) {
        return true
    }

    let find_provided_role = company_member_list
        .filter(user_uuid.eq(target_user_uuid)
        .and(company_uuid.eq_any(target_companies_uuids)
        .and(role_id.eq_any(need_roles_ids))))
        .limit(1)
        .execute(conn);

    match find_provided_role {
        Ok(provided_role) if provided_role == 1 => true,
        Ok(_) => false,
        Err(err) => {
            debug!("Failed get data: {:?}", err);
            false
        },
    }
}

/// Gets access type for company
pub(crate) fn get_access_type_company(
    target_company_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::company_ref::dsl::*;

    let type_access = company_ref
        .filter(uuid.eq(target_company_uuid))
        .select(type_access_id)
        .first::<i32>(conn);

    match type_access {
        Ok(ta) => Ok(ta),
        Err(err) => {
            debug!("Not found data: {:?}", err);
            Err(ServiceError::BadRequest(
                "Not found data".to_string()
            ))
        },
    }
}
