use crate::errors::ServiceError;
// use crate::models::user::model::LoggedUser;
use diesel::prelude::*;
use uuid::Uuid;

/// checking whether the company has a supplier's status
pub fn check_is_supplier(uuid_company: Uuid, conn: &PgConnection) -> Result<bool, ServiceError> {
    use crate::schema::company_ref::dsl::*;

    let get_company_status: bool = company_ref
        .filter(uuid.eq(uuid_company))
        .select(is_supplier)
        .first(conn)
        .unwrap_or(false);

    match get_company_status {
        true => Ok(true),
        // false => Ok(false),
        _ => Err(ServiceError::BadRequest(
            "The company is not supplier.".to_string(),
        )),
    }
}

/// checking the availability of the required access level
pub(crate) fn check_company_access(
    logged_uuid_user: Uuid,
    target_uuid_company: Uuid,
    required_access: i32,
    conn: &PgConnection,
) -> Result<bool, ServiceError> {
    use crate::schema::company_ref::dsl::*;

    // check user on owner company
    let user_owner_company = company_ref
        .filter(uuid.eq(target_uuid_company))
        .filter(uuid_user.eq(logged_uuid_user))
        .execute(conn)
        .unwrap_or(0);

    if user_owner_company == 1 {
        return Ok(true)
    }

    let found_id_type_access: i32 = get_id_type_access(
        company_member_role(
            logged_uuid_user,
            target_uuid_company,
            conn,
        ),
        conn
    );

    match found_id_type_access {
        1..=i32::MAX => {
            // debug!("get_id_type_access ({:?}) < required_access ({:?})", &get_id_type_access, &required_access);
            // delete represent and save delete data for send response
            if found_id_type_access < required_access {
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
    logged_uuid_user: Uuid,
    target_uuid_company: Uuid,
    conn: &PgConnection,
) -> i32 {
    use crate::schema::company_member_role::dsl::*;

    // find id_role user
    company_member_role
        .filter(uuid_company.eq(target_uuid_company))
        .filter(uuid_user.eq(logged_uuid_user))
        .select(id_role)
        .first(conn)
        .unwrap_or(0)
}

pub(crate) fn get_id_type_access(
    id_role_user: i32,
    conn: &PgConnection,
) -> i32 {
    use crate::schema::role_access::dsl::*;

    // find level access for id_role
    role_access
        .filter(id_role.eq(id_role_user))
        .select(id_type_access)
        .first(conn)
        .unwrap_or(0)
}
