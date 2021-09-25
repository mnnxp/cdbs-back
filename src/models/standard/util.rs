use crate::database::{get_conn, PooledConnection};
use crate::errors::{ServiceResult, ServiceError};
use crate::models::company as company;
use async_graphql::Context;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking have need access level
pub(crate) fn check_standard_access(
    cxt: &Context<'_>,
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    required_access: i32,
) -> ServiceResult<bool> {


    use crate::schema::standard_ref::dsl::*;

    // check user on owner standard
    let user_owner_standard = standard_ref
        .filter(uuid.eq(target_standard_uuid))
        .filter(user_uuid.eq(target_user_uuid))
        .execute(conn)
        .unwrap_or(0);

    if user_owner_standard == 1 {
        return Ok(true)
    }

    // search user access level to the standard
    let found_type_access_id: i32 = get_user_access_standard(
        target_user_uuid,
        target_standard_uuid,
        conn,
    );


    debug!("0 < found_type_access_id {:?} <= required_access {:?}", found_type_access_id, required_access);
    // return true if level is equal or higher than required
    if 0 < found_type_access_id && found_type_access_id <= required_access {
        return Ok(true)
    }

    // recursive search for access from companies (owner, member) with access
    let find_max_level = recursive_search_availability_access(
        target_user_uuid,
        target_standard_uuid,
        required_access,
        conn,
    );

    debug!("find_max_level {:?}", find_max_level);
    match find_max_level {
        1..=i32::MAX if found_type_access_id <= required_access => {
            Ok(true)
        },
        // 1..=i32::MAX => {
        //     Err(ServiceError::BadRequest(
        //         "You have insufficient access level.".to_string(),
        //     ))
        // },
        _ => Err(ServiceError::BadRequest("You not have access.".to_string())),
    }
}

/// Search and returns the user's personal access level
pub(crate) fn get_user_access_standard(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    conn: &PgConnection,
) -> i32 {
    use crate::schema::user_access_to_standard::dsl::*;

    // find role_id user
    user_access_to_standard
        .filter(standard_uuid.eq(target_standard_uuid))
        .filter(user_uuid.eq(target_user_uuid))
        .select(type_access_id)
        .first(conn)
        .unwrap_or(0)
}

/// Search and returns the user's personal access level
pub(crate) fn recursive_search_availability_access(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    required_access: i32,
    conn: &PgConnection,
) -> i32 {
    let find_access_of_companies = get_access_from_company(
        target_user_uuid,
        target_standard_uuid,
        required_access,
        conn,
    );

    // find user member companies
    match find_access_of_companies {
        find_access if (required_access - find_access) < 0 => {
            debug!("found access in recursive (owned): {:?}", find_access);
            find_access
        },
        find_access => {
            debug!("not found access in recursive (owned): {:?}", find_access);
            get_access_granted_company(
                target_user_uuid,
                target_standard_uuid,
                required_access,
                conn,
            )
        },
    }
}

// Search companies that have access to standard
// returns uuid companies and id access
// pub(crate) fn get_companies_access_standard(
//     target_standard_uuid: &Uuid,
//     conn: &PgConnection,
// ) -> Vec<(Uuid, i32)> {
//     use crate::schema::company_access_to_standard::dsl::*;
//
//     // find companies that have access to standard
//     company_access_to_standard
//         .filter(standard_uuid.eq(target_standard_uuid))
//         .select((
//             company_uuid,
//             type_access_id
//         ))
//         .load::<(Uuid, i32)>(conn)
//         .unwrap_or_default()
// }

/// Search companies that have a need-level access to standard
/// returns found type_access_id
pub(crate) fn get_access_granted_company(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    required_access: i32,
    conn: &PgConnection,
) -> i32 {
    use diesel::sql_types::Integer;

    #[derive(Debug, QueryableByName)]
    pub struct RoleAccess {
        #[sql_type = "Integer"]
        access_level: i32,
    }

    debug!("target_user_uuid: {:?}", target_user_uuid);
    debug!("target_standard_uuid: {:?}", target_standard_uuid);
    debug!("required_access: {:?}", required_access);

    let query: &str = "SELECT  \
        CASE \
            WHEN company_access_to_standard.type_access_id > role_access.type_access_id \
                THEN company_access_to_standard.type_access_id \
                ELSE role_access.type_access_id \
        END access_level \
        FROM company_member_role \
        INNER JOIN company_access_to_standard \
            ON (company_member_role.company_uuid = company_access_to_standard.company_uuid) \
        INNER JOIN role_access \
            ON (company_member_role.role_id = role_access.role_id) \
        WHERE company_member_role.user_uuid = $1 \
            AND company_access_to_standard.standard_uuid = $2 \
            AND company_access_to_standard.type_access_id <= $3 \
            AND role_access.type_access_id <= $3;";

    let find_user_access = diesel::sql_query(query)
        .bind::<diesel::sql_types::Uuid, _>(target_user_uuid)
        .bind::<diesel::sql_types::Uuid, _>(target_standard_uuid)
        .bind::<diesel::sql_types::Integer, _>(required_access)
        .get_result::<RoleAccess>(conn);

    debug!("find_user_access: {:?}", find_user_access);

    match find_user_access {
        Ok(find_user_access) => find_user_access.access_level,
        Err(_) => 0,
    }
}

/// Search a company with user owned and have level
/// returns found access need
pub(crate) fn get_access_from_company(
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    required_access: i32,
    conn: &PgConnection,
) -> i32 {
    use crate::schema::company_access_to_standard::dsl::*;

    // find user companies
    let target_company_uuid = company::util::get_companies_owned_by_user(
        target_user_uuid,
        conn
    );

    // find companies that have a need-level access to standard
    company_access_to_standard
        .filter(standard_uuid.eq(target_standard_uuid))
        .filter(company_uuid.eq_any(target_company_uuid))
        .filter(type_access_id.gt(required_access))
        .select(type_access_id)
        .first(conn)
        .unwrap_or(0)
}

/// Gets the default access for target standard
pub(crate) fn get_access_set(
    cxt: &Context<'_>,
    target_standard_uuid: &Uuid,
) -> i32 {
    use crate::schema::standard_ref::dsl::*;

    // check default access for standard
    standard_ref
        .filter(uuid.eq(target_standard_uuid))
        .select(type_access_id)
        .first(conn)
        .unwrap_or(0)
}
