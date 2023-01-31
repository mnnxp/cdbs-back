use crate::errors::ServiceError;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking whether the company has a supplier's status
pub(crate) fn check_is_supplier(
    target_company_uuid: &Uuid,
    conn: &mut PgConnection
) -> Result<bool, ServiceError> {
    use crate::schema::company_ref::dsl::*;

    let get_company_status = company_ref
        .filter(uuid.eq(target_company_uuid))
        .select(is_supplier)
        .first::<bool>(conn);

    match get_company_status {
        Ok(true) => Ok(true),
        Ok(false) => Err(ServiceError::BadRequest(
            "The company is not supplier.".to_string(),
        )),
        _ => Err(ServiceError::BadRequest(
            "Failed check data".to_string(),
        )),
    }
}

// Search for owned companies
// pub(crate) fn get_companies_owned_by_user(
//     target_user_uuid: &Uuid,
//     conn: &mut PgConnection,
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
//     conn: &mut PgConnection,
// ) -> Vec<(Uuid, i32)> {
//     use crate::schema::company_member_list::dsl::*;
//
//     // find role_id user
//     company_member_list
//         .filter(user_uuid.eq(target_user_uuid))
//         .select((
//             company_uuid,
//             role_id
//         ))
//         .load(conn)
//         .unwrap_or_default()
// }
