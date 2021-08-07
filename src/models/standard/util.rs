use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceError;
use crate::models::company as company;
use async_graphql::Context;
use diesel::prelude::*;
use uuid::Uuid;

/// checking the availability of the required access level
pub(crate) fn check_standard_access(
    context: &Context<'_>,
    target_uuid_user: Uuid,
    target_uuid_standard: Uuid,
    required_access: i32,
) -> Result<bool, ServiceError> {
    let conn: &PooledConnection = &get_conn(context)?;

    use crate::schema::standard_ref::dsl::*;

    // check user on owner standard
    let user_owner_standard = standard_ref
        .filter(uuid.eq(target_uuid_standard))
        .filter(uuid_user.eq(target_uuid_user))
        .execute(conn)
        .unwrap_or(0);

    if user_owner_standard == 1 {
        return Ok(true)
    }

    // search user access level to the standard
    let found_id_type_access: i32 = get_user_access_standard(
        target_uuid_user,
        target_uuid_standard,
        conn,
    );


    debug!("0 < found_id_type_access {:?} <= required_access {:?}", found_id_type_access, required_access);
    // return true if level is equal or higher than required
    if 0 < found_id_type_access && found_id_type_access <= required_access {
        return Ok(true)
    }

    // recursive search for access from companies (owner, member) with access
    let find_max_level = recursive_search_availability_access(
        target_uuid_user,
        target_uuid_standard,
        required_access,
        conn,
    );

    debug!("find_max_level {:?}", find_max_level);
    match find_max_level {
        1..=i32::MAX if found_id_type_access < required_access => {
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
    target_uuid_user: Uuid,
    target_uuid_standard: Uuid,
    conn: &PgConnection,
) -> i32 {
    use crate::schema::user_access_to_standard::dsl::*;

    // find id_role user
    user_access_to_standard
        .filter(uuid_standard.eq(target_uuid_standard))
        .filter(uuid_user.eq(target_uuid_user))
        .select(id_type_access)
        .first(conn)
        .unwrap_or(0)
}

/// Search and returns the user's personal access level
pub(crate) fn recursive_search_availability_access(
    target_uuid_user: Uuid,
    target_uuid_standard: Uuid,
    required_access: i32,
    conn: &PgConnection,
) -> i32 {
    let find_access_of_companies = get_companies_owned_by_user(
        target_uuid_user,
        target_uuid_standard,
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
            get_user_access_granted_by_company(
                target_uuid_user,
                target_uuid_standard,
                required_access,
                conn,
            )
        },
    }
}

/// Search companies that have access to standard
/// returns uuid companies and id access
pub(crate) fn _get_companies_access_standard(
    target_uuid_standard: Uuid,
    conn: &PgConnection,
) -> Vec<(Uuid, i32)> {
    use crate::schema::company_access_to_standard::dsl::*;

    // find companies that have access to standard
    company_access_to_standard
        .filter(uuid_standard.eq(target_uuid_standard))
        .select((
            uuid_company,
            id_type_access
        ))
        .load::<(Uuid, i32)>(conn)
        .unwrap_or_default()
}

/// Search companies that have a need-level access to standard
/// returns id_type_access
pub(crate) fn get_user_access_granted_by_company(
    target_uuid_user: Uuid,
    target_uuid_standard: Uuid,
    required_access: i32,
    conn: &PgConnection,
) -> i32 {
    use diesel::sql_types::Integer;
    #[derive(QueryableByName)]
    pub struct RoleAccess {
        #[sql_type = "Integer"]
        id_type_access: i32,
    }

    debug!("target_uuid_user: {:?}", target_uuid_user);
    debug!("target_uuid_standard: {:?}", target_uuid_standard);
    debug!("required_access: {:?}", required_access);

    let query: &str = "SELECT role_access.id_type_access \
        FROM company_member_role \
        INNER JOIN company_access_to_standard \
            ON (company_member_role.uuid_company = company_access_to_standard.uuid_company) \
        INNER JOIN role_access \
            ON (company_member_role.id_role = role_access.id_role) \
        WHERE company_member_role.uuid_user = $1 \
            AND company_access_to_standard.uuid_standard = $2 \
            AND role_access.id_type_access <= $3;";

    let test = diesel::sql_query(query)
        .bind::<diesel::sql_types::Uuid, _>(&target_uuid_user)
        .bind::<diesel::sql_types::Uuid, _>(&target_uuid_standard)
        .bind::<diesel::sql_types::Integer, _>(required_access)
        .get_result::<RoleAccess>(conn);

    match test {
        Ok(test) => test.id_type_access,
        Err(_) => 0,
    }
}

/// Get user owned company then have role_access
pub(crate) fn get_companies_owned_by_user(
    target_uuid_user: Uuid,
    target_uuid_standard: Uuid,
    required_access: i32,
    conn: &PgConnection,
) -> i32 {
    use crate::schema::company_access_to_standard::dsl::*;

    // find user companies
    let target_uuid_company = company::util::get_companies_owned_by_user(
        target_uuid_user,
        conn
    );

    // find companies that have a need-level access to standard
    company_access_to_standard
        .filter(uuid_standard.eq(target_uuid_standard))
        .filter(uuid_company.eq_any(target_uuid_company))
        .filter(id_type_access.gt(required_access))
        .select(id_type_access)
        .first(conn)
        .unwrap_or(0)
}

/// returns the value of the id_type_access
/// field set for the standard by default
pub(crate) fn get_default_access_standard(
    context: &Context<'_>,
    target_uuid_standard: Uuid,
) -> Result<i32, ServiceError> {
    use crate::schema::standard_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    // check default access for standard
    Ok(standard_ref
        .filter(uuid.eq(target_uuid_standard))
        .select(id_type_access)
        .first(conn)
        .unwrap_or(0))
}
