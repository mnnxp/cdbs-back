use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

/// checking whether the component has flag is_base
pub fn check_is_base(
    target_component_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::component_ref::dsl::*;

    let get_component_status = component_ref
        .filter(uuid.eq(target_component_uuid)
        .and(is_base.eq(true)))
        .limit(1)
        .execute(conn);

    match get_component_status {
        Ok(count) if count == 1 => Ok(true),
        Ok(_) => Err(ServiceError::BadRequest(
            "The component is not standard.".to_string(),
        )),
        _ => Err(ServiceError::BadRequest(
            "Failed check data".to_string(),
        )),
    }
}

/// Checking component owner, return bool
pub fn check_is_owner(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    conn: &PgConnection
) -> bool {
    use crate::schema::component_ref::dsl::*;

    let check_owner_component = component_ref
        .filter(user_uuid.eq(target_user_uuid)
        .and(uuid.eq(target_component_uuid)))
        .limit(1)
        .execute(conn);

    match check_owner_component {
        Ok(count) if count == 1 => true,
        Ok(_) => false,
        Err(err) => {
            debug!("Failed check data: {:?}", err);
            // Err(ServiceError::BadRequest("Failed check data".to_string()))
            false
        },
    }
}

/// Checking onwed component
/// Return error if user not owned
pub fn check_is_owner_with_err(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    match check_is_owner(target_user_uuid, target_component_uuid, conn) {
        true => Ok(true),
        false => Err(ServiceError::BadRequest(
            "Access denied".to_string(),
        )),
    }
}

/// Full find and check access to component for user
pub(crate) fn check_access_component_for_user(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    need_access_level: &i32,
    ownership_check: bool,
    conn: &PgConnection
) -> ServiceResult<bool> {
    // 1. проверить владение компонентом
    // если флаг ownership_check true

    // ownership check for ownership_check is true
    if ownership_check && check_is_owner(target_user_uuid, target_component_uuid, conn) {
        return Ok(true)
    }

    // 2. проверить наличие доступа к компоненту,
    // установленного в user_access_to_component
    // check if the user has personal access to the component
    if check_user_access_to_component(
        target_user_uuid,
        target_component_uuid,
        need_access_level,
        conn
    ) {
        return Ok(true)
    }

    // 3. рекурсивно проверить доступ у компаний,
    // 3.1 которые предоставляют требуемый доступ пользователю:
    // 3.2 получить список компаний с подходящим доступом company_access_to_component
    // 3.3 получить списко ролей с подходящим доступом role_access
    // 3.4 поиск пользователя среди сотрудников компаний в company_member_role с подходящей ролью:
    // фильтр пользователя, список компаний, список ролей)
    // checking the availability of user access provided by the company
    if check_user_access_provided_by_company(target_user_uuid,
        target_component_uuid,
        need_access_level,
        conn
    )? {
        return Ok(true)
    };

    // not found need access level for target user
    Err(ServiceError::BadRequest(
        "Access denied".to_string()
    ))
}

/// Checking the required level of user access to the component
pub(crate) fn check_user_access_to_component(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    need_access_level: &i32,
    conn: &PgConnection
) -> bool {
    use crate::schema::user_access_to_component::dsl::*;
    // 2. проверить наличие доступа к компоненту,
    // установленного в user_access_to_component

    let check_res = user_access_to_component
        .filter(component_uuid.eq(target_component_uuid)
        .and(user_uuid.eq(target_user_uuid)
        .and(type_access_id.le(need_access_level)))) // <-- access < or = need_access_level
        .limit(1)
        .execute(conn);

    match check_res {
        Ok(count) if count == 1 => true,
        Ok(_) => false,
        Err(err) => {
            debug!("Failed check data: {:?}", err);
            // Err(ServiceError::BadRequest("Failed check data".to_string()))
            false
        },
    }
}

/// Сhecking the availability of user access provided by the company
pub(crate) fn check_user_access_provided_by_company(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    need_access_level: &i32,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::models::company::util::get_roles_ids_for_access;
    use crate::models::company::util::check_clerk_with_suitable_role;

    // 3. рекурсивно проверить доступ у компаний,
    // 3.1 получить список которые предоставляют требуемый доступ пользователю:
    // 3.2 получить список компаний с подходящим доступом company_access_to_component
    // get a list of companies that have access to a component
    let target_companis_uuids = get_companies_have_access_to_component(
        target_component_uuid,
        need_access_level,
        conn
    )?;

    // 3.3 получить список ролей с подходящим доступом role_access
    // 3.4 поиск пользователя среди сотрудников компаний в company_member_role с подходящей ролью:
    // фильтр пользователя, список компаний, список ролей)
    if check_clerk_with_suitable_role(
        target_user_uuid,
        &target_companis_uuids,
        &get_roles_ids_for_access(need_access_level, conn)?,
        conn
    ) {
        return Ok(true)
    }

    Ok(false)
}

/// Gets list of companies that have need level access to a component
pub(crate) fn get_companies_have_access_to_component(
    target_component_uuid: &Uuid,
    need_access_level: &i32,
    conn: &PgConnection
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::company_access_to_component::dsl::*;

    // 3.2 получить список компаний
    // с подходящим доступом company_access_to_component

    let companies_uuids = company_access_to_component
        .filter(component_uuid.eq(target_component_uuid)
        .and(type_access_id.le(need_access_level))) // <-- access < or = need_access_level
        .select(company_uuid)
        .load(conn);

    match companies_uuids {
        Ok(cs_uuids) if !cs_uuids.is_empty() => Ok(cs_uuids),
        // not found companies with need access
        Ok(_) => Err(ServiceError::BadRequest(
            "Access denied".to_string()
        )),
        Err(err) => {
            debug!("Failed check data: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed check data".to_string()
            ))
        },
    }
}
