use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceError;
use crate::models::company as company;
use async_graphql::Context;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking have need access level
pub(crate) fn check_standard_access(
    context: &Context<'_>,
    target_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    required_access: i32,
) -> Result<bool, ServiceError> {
    let conn: &PooledConnection = &get_conn(context)?;

    use crate::schema::standard_ref::dsl::*;

    // check user on owner standard
    let user_owner_standard = standard_ref
        .filter(uuid.eq(target_standard_uuid))
        .filter(uuid_user.eq(target_user_uuid))
        .execute(conn)
        .unwrap_or(0);

    if user_owner_standard == 1 {
        return Ok(true)
    }

    // search user access level to the standard
    let found_id_type_access: i32 = get_user_access_standard(
        target_user_uuid,
        target_standard_uuid,
        conn,
    );


    debug!("0 < found_id_type_access {:?} <= required_access {:?}", found_id_type_access, required_access);
    // return true if level is equal or higher than required
    if 0 < found_id_type_access && found_id_type_access <= required_access {
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
        1..=i32::MAX if found_id_type_access <= required_access => {
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

    // find id_role user
    user_access_to_standard
        .filter(uuid_standard.eq(target_standard_uuid))
        .filter(uuid_user.eq(target_user_uuid))
        .select(id_type_access)
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
//         .filter(uuid_standard.eq(target_standard_uuid))
//         .select((
//             uuid_company,
//             id_type_access
//         ))
//         .load::<(Uuid, i32)>(conn)
//         .unwrap_or_default()
// }

/// Search companies that have a need-level access to standard
/// returns found id_type_access
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
            WHEN company_access_to_standard.id_type_access > role_access.id_type_access \
                THEN company_access_to_standard.id_type_access \
                ELSE role_access.id_type_access \
        END access_level \
        FROM company_member_role \
        INNER JOIN company_access_to_standard \
            ON (company_member_role.uuid_company = company_access_to_standard.uuid_company) \
        INNER JOIN role_access \
            ON (company_member_role.id_role = role_access.id_role) \
        WHERE company_member_role.uuid_user = $1 \
            AND company_access_to_standard.uuid_standard = $2 \
            AND company_access_to_standard.id_type_access <= $3 \
            AND role_access.id_type_access <= $3;";

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
        .filter(uuid_standard.eq(target_standard_uuid))
        .filter(uuid_company.eq_any(target_company_uuid))
        .filter(id_type_access.gt(required_access))
        .select(id_type_access)
        .first(conn)
        .unwrap_or(0)
}

/// Gets the default access for target standard
pub(crate) fn get_access_set(
    context: &Context<'_>,
    target_standard_uuid: &Uuid,
) -> Result<i32, ServiceError> {
    use crate::schema::standard_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    // check default access for standard
    Ok(standard_ref
        .filter(uuid.eq(target_standard_uuid))
        .select(id_type_access)
        .first(conn)
        .unwrap_or(0))
}
