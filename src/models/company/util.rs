use crate::errors::{ServiceResult, ServiceError};
// use crate::models::company::model::Company;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking companies for user owner,
/// return true if user owner any of companies
pub fn check_is_owner(
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
            // Err(ServiceError::BadRequest("Failed check data".to_string()))
            false
        },
    }
}

/// checking whether the company has a supplier's status
pub fn check_is_supplier(
    target_company_uuid: &Uuid,
    conn: &PgConnection
) -> Result<bool, ServiceError> {
    use crate::schema::company_ref::dsl::*;

    let get_company_status = company_ref
        .filter(uuid.eq(target_company_uuid)
        .and(is_supplier.eq(true)))
        .limit(1)
        .execute(conn);

    match get_company_status {
        Ok(count) if count == 1 => Ok(true),
        Ok(_) => Err(ServiceError::BadRequest(
            "The company is not supplier.".to_string(),
        )),
        _ => Err(ServiceError::BadRequest(
            "Failed check data".to_string(),
        )),
    }
}

/// checking the availability of the required access level
pub(crate) fn check_company_access(
    target_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    required_access: i32,
    conn: &PgConnection,
) -> Result<bool, ServiceError> {
    use crate::schema::company_ref::dsl::*;

    // check user on owner company
    let user_owner_company = company_ref
        .filter(uuid.eq(target_company_uuid))
        .filter(user_uuid.eq(target_user_uuid))
        .execute(conn)
        .unwrap_or(0);

    if user_owner_company == 1 {
        return Ok(true)
    }

    let found_type_access_id: i32 = get_type_access_id(
        company_member_role(
            target_user_uuid,
            target_company_uuid,
            conn,
        ),
        conn
    );

    match found_type_access_id {
        1..=i32::MAX => {
            // debug!("get_type_access_id ({:?}) < required_access ({:?})", &get_type_access_id, &required_access);
            if found_type_access_id < required_access {
                Ok(true)
            } else {
                Err(ServiceError::BadRequest(
                    "You have insufficient access level.".to_string(),
                ))
            }
        }
        _ => Err(ServiceError::BadRequest("You not have access.".to_string())),
    }
}

pub(crate) fn company_member_role(
    target_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    conn: &PgConnection,
) -> i32 {
    use crate::schema::company_member_role::dsl::*;

    // find role_id user
    company_member_role
        .filter(company_uuid.eq(target_company_uuid))
        .filter(user_uuid.eq(target_user_uuid))
        .select(role_id)
        .first(conn)
        .unwrap_or(0)
}

/// Search type_access_id for target role_id
pub(crate) fn get_type_access_id(
    target_role_id: i32,
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
    need_level_access: &i32,
    conn: &PgConnection
) -> ServiceResult<Vec<i32>> {
    use crate::schema::role_access::dsl::*;
    // 3.3 получить список ролей с подходящим доступом role_access

    let roles_ids = role_access
        .filter(type_access_id.le(need_level_access)) // <-- filter access < or = need_level_access
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
    use crate::schema::company_member_role::dsl::*;

    // if user owner any of companies
    if check_is_owner(target_user_uuid, target_companies_uuids, conn) {
        return true
    }

    // 3.4 поиск пользователя среди сотрудников компаний в company_member_role с подходящей ролью:
    // фильтр пользователя, список компаний, список ролей)

    let find_provided_role = company_member_role
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


// Search for owned companies
// pub(crate) fn get_companies_owned_by_user(
//     target_user_uuid: &Uuid,
//     conn: &PgConnection,
// ) -> Vec<Uuid> {
//     use crate::schema::company_ref::dsl::*;
//
//     company_ref
//         .filter(user_uuid.eq(target_user_uuid))
//         .select(uuid)
//         .load(conn)
//         .unwrap_or_default()
// }

// Search for companies the user belongs to
// pub(crate) fn get_companies_with_member_by_user(
//     target_user_uuid: &Uuid,
//     conn: &PgConnection,
// ) -> Vec<(Uuid, i32)> {
//     use crate::schema::company_member_role::dsl::*;
//
//     // find role_id user
//     company_member_role
//         .filter(user_uuid.eq(target_user_uuid))
//         .select((
//             company_uuid,
//             role_id
//         ))
//         .load(conn)
//         .unwrap_or_default()
// }
