use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

/// Get access type for user
pub(crate) fn get_access_type_user(
    target_user_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::user_ref::dsl as user_ref;

    let get_access = user_ref::user_ref
        .filter(user_ref::uuid.eq(target_user_uuid))
        .select(user_ref::type_access_id)
        .first::<i32>(conn);

    match get_access {
        Ok(x) => Ok(x),
        Err(err) => {
            debug!("Failed get access for user: {:?}", err);

            Err(ServiceError::InternalServerError)
        },
    }
}

/// Check access to user for logged user
pub(crate) fn check_access_user_for_user(
    logged_user_uuid: &Uuid,
    target_user_uuid: &Uuid,
    need_access_level: &i32,
    conn: &PgConnection
) -> ServiceResult<bool> {
    // get set type access for target user
    let access_type_user = get_access_type_user(
        target_user_uuid,
        conn
    )?;

    // if request to view a public user
    if need_access_level == &3 {
        // if target user public
        if access_type_user == 3 {
            return Ok(true)
        }
    }

    // checking if user have access
    // to component of target user
    if user_have_access_component_user(
        logged_user_uuid,
        target_user_uuid,
        conn
    ) {
        return Ok(true)
    };

    // checking if target user have access
    // to component of user
    if user_have_access_component_user(
        logged_user_uuid,
        target_user_uuid,
        conn
    ) {
        return Ok(true)
    };

    // checking if user have access
    // to standard of target user
    if user_have_access_standard_user(
        logged_user_uuid,
        target_user_uuid,
        conn
    ) {
        return Ok(true)
    };

    // checking if target user have access
    // to standard of user
    if user_have_access_standard_user(
        target_user_uuid,
        logged_user_uuid,
        conn
    ) {
        return Ok(true)
    };

    // checking if users are members of the same company
    if users_has_one_company(
        logged_user_uuid,
        target_user_uuid,
        conn
    ) {
        return Ok(true)
    };

    // checking if the user
    // is a member of the target user's company
    if member_in_company_user(
        logged_user_uuid,
        target_user_uuid,
        conn
    ) {
        return Ok(true)
    };

    // second checking if target user
    // is a member company with owner logged user
    if member_in_company_user(
        target_user_uuid,
        logged_user_uuid,
        conn
    ) {
        return Ok(true)
    };

    // not found need access level for target user
    Err(ServiceError::BadRequest(
        "Access denied".to_string()
    ))
}

/// Check users for membering in one company
fn users_has_one_company(
    logged_user_uuid: &Uuid,
    target_user_uuid: &Uuid,
    conn: &PgConnection,
) -> bool {
    use crate::schema::company_member_list::dsl as company_member_list;

    // get companies for first user
    let target_companies = company_member_list::company_member_list
        .filter(company_member_list::user_uuid.eq(target_user_uuid))
        .select(company_member_list::company_uuid)
        .load::<Uuid>(conn)
        .expect("Failed get companies data from database");

    // check second user in companies of list for first user
    let res_check = company_member_list::company_member_list
        .filter(company_member_list::user_uuid.eq(logged_user_uuid)
        .and(company_member_list::company_uuid.eq_any(&target_companies)))
        .limit(1)
        .execute(conn)
        .expect("Failed check companies for user on database");

    matches!(res_check, 1_usize)
}

/// Check if logged user is a member of target user company
fn member_in_company_user(
    logged_user_uuid: &Uuid,
    target_user_uuid: &Uuid,
    conn: &PgConnection,
) -> bool {
    use crate::schema::company_ref::dsl as company_ref;
    use crate::schema::company_member_list::dsl as company_member_list;

    // get companies with target user owner
    let target_companies = company_ref::company_ref
        .filter(company_ref::user_uuid.eq(target_user_uuid))
        .select(company_ref::uuid)
        .load::<Uuid>(conn)
        .expect("Failed get companies data from database");

    // check logged user in members target companies
    let res_check = company_member_list::company_member_list
        .filter(company_member_list::user_uuid.eq(logged_user_uuid)
        .and(company_member_list::company_uuid.eq_any(&target_companies)))
        .limit(1)
        .execute(conn)
        .expect("Failed check members on database");

    matches!(res_check, 1_usize)
}

/// Check user have access to component other user
fn user_have_access_component_user(
    logged_user_uuid: &Uuid,
    target_user_uuid: &Uuid,
    conn: &PgConnection,
) -> bool {
    use crate::schema::component_ref::dsl as component_ref;
    use crate::schema::user_access_to_component::dsl as user_access_to_component;

    // get components with ownership target user
    let target_components = component_ref::component_ref
        .filter(component_ref::user_uuid.eq(target_user_uuid))
        .select(component_ref::uuid)
        .load::<Uuid>(conn)
        .expect("Failed get components data from database");

    // check logged user have access to one of ownership target user components
    let res_check = user_access_to_component::user_access_to_component
        .filter(user_access_to_component::user_uuid.eq(logged_user_uuid)
        .and(user_access_to_component::component_uuid.eq_any(&target_components)))
        .limit(1)
        .execute(conn)
        .expect("Failed check components for user on database");

    matches!(res_check, 1_usize)
}

/// Check user have access to standard other user
fn user_have_access_standard_user(
    logged_user_uuid: &Uuid,
    target_user_uuid: &Uuid,
    conn: &PgConnection,
) -> bool {
    use crate::schema::standard_ref::dsl as standard_ref;
    use crate::schema::user_access_to_standard::dsl as user_access_to_standard;

    // get standards with ownership target user
    let target_standards = standard_ref::standard_ref
        .filter(standard_ref::user_uuid.eq(target_user_uuid))
        .select(standard_ref::uuid)
        .load::<Uuid>(conn)
        .expect("Failed get standards from database");

    // check logged user have access to one of ownership target user standards
    let res_check = user_access_to_standard::user_access_to_standard
        .filter(user_access_to_standard::user_uuid.eq(logged_user_uuid)
        .and(user_access_to_standard::standard_uuid.eq_any(&target_standards)))
        .limit(1)
        .execute(conn)
        .expect("Failed check standards for user on database");

    matches!(res_check, 1_usize)
}
