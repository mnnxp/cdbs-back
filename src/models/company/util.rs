use crate::errors::ServiceError;
use crate::models::user::model::LoggedUser;
use diesel::prelude::*;
use uuid::Uuid;

// pub fn verify_uuid_user(user: &LoggedUser, uuid_user: Uuid) -> Result<bool, ServiceError> {
//     match user.0 {
//         None => Err(ServiceError::Unauthorized),
//         Some(ref user) if user.uuid == uuid_user => Ok(true),
//         _ => Err(ServiceError::BadRequest("Uuid not correct.".to_string())),
//         // Some(ref user) => Err(ServiceError::BadRequest(format!("Uuid not correct. UUID1: {}, UUID2: {};", user.uuid, uuid_user))),
//     }
// }

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

pub(crate) fn check_company_access(
    user: &LoggedUser,
    input_uuid_company: Uuid,
    required_access: i32,
    conn: &PgConnection,
) -> Result<bool, ServiceError> {
    match user.0 {
        None => Err(ServiceError::Unauthorized),
        Some(ref user) => {
            use crate::schema::company_member_role::dsl::id_role as member_id_role;
            use crate::schema::company_member_role::dsl::*;
            use crate::schema::role_access::dsl::id_role as access_id_role;
            use crate::schema::role_access::dsl::*;
            // use crate::schema::company_ref::dsl::*;

            // check user on owner company
            // let get_id_role_user: i32 = company_member_role
            //     .filter(uuid_company.eq(uuid_company))
            //     .filter(uuid_user.eq(uuid_user))
            //     .select(member_id_role)
            //     .first(conn)
            //     .unwrap_or(0);

            // find id_role user
            let get_id_role_user: i32 = company_member_role
                .filter(uuid_company.eq(input_uuid_company))
                .filter(uuid_user.eq(user.uuid))
                .select(member_id_role)
                .first(conn)
                .unwrap_or(0);

            // find level access for id_role
            let get_id_type_access: i32 = role_access
                .filter(access_id_role.eq(get_id_role_user))
                .select(id_type_access)
                .first(conn)
                .unwrap_or(0);

            match get_id_type_access {
                1..=i32::MAX => {
                    // debug!("get_id_type_access ({:?}) < required_access ({:?})", &get_id_type_access, &required_access);
                    // delete represent and save delete data for send response
                    if get_id_type_access < required_access {
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
        // _ => Err(ServiceError::BadRequest("Uuid not correct.".to_string())),
        // Some(ref user) => Err(ServiceError::BadRequest(format!("Uuid not correct. UUID1: {}, UUID2: {};", user.uuid, uuid_user))),
    }
}
