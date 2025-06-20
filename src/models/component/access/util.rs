use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use diesel::prelude::*;
use uuid::Uuid;

/// Checking component owner, return bool
pub(crate) fn check_is_owner(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::component_ref::dsl::*;

    let check_owner_component = component_ref
        .filter(
            user_uuid
                .eq(target_user_uuid)
                .and(uuid.eq(target_component_uuid)),
        )
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check owner component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(check_owner_component == 1)
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
/// return err if not found need access
pub(crate) fn check_access_component_for_user(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    need_access_level: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить владение компонентом
    // если флаг ownership_check true

    // if request to view a public component
    if need_access_level == &3 {
        let access_type_component = get_access_type_component(target_component_uuid, conn)?;
        // if target component public
        if access_type_component == 3 {
            return Ok(true);
        }
    }

    // ownership check for ownership_check is true
    if check_is_owner(target_user_uuid, target_component_uuid, conn)? {
        return Ok(true);
    }

    // 2. проверить наличие доступа к компоненту,
    // установленного в user_access_to_component
    // check if the user has personal access to the component
    if check_user_access_to_component(
        target_user_uuid,
        target_component_uuid,
        need_access_level,
        conn,
    )? {
        return Ok(true);
    }

    // 3. рекурсивно проверить доступ у компаний,
    // 3.1 которые предоставляют требуемый доступ пользователю:
    // 3.2 получить список компаний с подходящим доступом company_access_to_component
    // 3.3 получить списко ролей с подходящим доступом role_access
    // 3.4 поиск пользователя среди сотрудников компаний в company_member_list с подходящей ролью:
    // фильтр пользователя, список компаний, список ролей)
    // checking the availability of user access provided by the company
    match check_user_access_provided_by_company(
        target_user_uuid,
        target_component_uuid,
        need_access_level,
        conn,
    )? {
        true => Ok(true),
        // not found need access level for target user
        false => Err(get_err_msg(ErrorMessage::AccessDenied)),
    }
}

/// Checking the required level of user access to the component
pub(crate) fn check_user_access_to_component(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    need_access_level: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::user_access_to_component::dsl::*;
    // 2. проверить наличие доступа к компоненту,
    // установленного в user_access_to_component

    let check_res = user_access_to_component
        .filter(
            component_uuid
                .eq(target_component_uuid)
                .and(user_uuid.eq(target_user_uuid)),
        )
        .select(type_access_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check access component for user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(matches!(check_res.first(), Some(x) if need_access_level >= x))
}

/// Сhecking the availability of user access provided by the company
pub(crate) fn check_user_access_provided_by_company(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    need_access_level: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::models::company::access::util::check_clerk_with_suitable_role;
    use crate::models::company::access::util::get_roles_ids_for_access;

    // 3. рекурсивно проверить доступ у компаний,
    // 3.1 получить список которые предоставляют требуемый доступ пользователю:
    // 3.2 получить список компаний с подходящим доступом company_access_to_component
    // get a list of companies that have access to a component
    let target_companis_uuids =
        get_companies_have_access_to_component(target_component_uuid, need_access_level, conn)?;

    // 3.3 получить список ролей с подходящим доступом role_access
    // 3.4 поиск пользователя среди сотрудников компаний в company_member_list с подходящей ролью:
    // фильтр пользователя, список компаний, список ролей)
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
    need_access_level: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::company_access_to_component::dsl::*;

    // 3.2 получить список компаний
    // с подходящим доступом company_access_to_component

    let companies_uuids = company_access_to_component
        .filter(
            component_uuid
                .eq(target_component_uuid)
                .and(type_access_id.le(need_access_level)),
        ) // <-- access < or = need_access_level
        .select(company_uuid)
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
    use crate::schema::component_ref::dsl::*;

    component_ref
        .filter(uuid.eq(target_component_uuid))
        .select(type_access_id)
        .first::<i32>(conn)
        .map_err(|err| {
            debug!("Not found data: {:?}", err);
            ServiceError::InternalServerError
        })
}
