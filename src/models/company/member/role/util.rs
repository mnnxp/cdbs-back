use crate::errors::{ServiceResult, ServiceError};
use diesel::{PgConnection, prelude::*};
use uuid::Uuid;

/// Get company uuid by role id
pub(crate) fn get_company_by_role(
    target_role_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    use crate::schema::role_member_list::dsl::*;

    let res = role_member_list
        .filter(id.eq(target_role_id))
        .select(company_uuid)
        .first::<Uuid>(conn);

    match res {
        Ok(x) => Ok(x),
        Err(err) => {
            debug!("Role not found: {:?}", err);
            Err(ServiceError::BadRequest(
                "Role not found".to_string()
            ))
        }
    }
}

/// Check role for company
/// return error if not found role for company
pub(crate) fn check_role_of_company(
    target_company_uuid: &Uuid,
    target_role_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::role_member_list::dsl::*;

    let res = role_member_list
        .filter(company_uuid.eq(target_company_uuid)
        .and(id.eq(target_role_id)))
        .execute(conn);

    match res {
        Ok(1_usize) => Ok(true),
        Ok(x) => {
            debug!("Role not found or found: {:?}", x);
            Err(ServiceError::BadRequest(
                "Role not found".to_string()
            ))
        },
        Err(err) => {
            debug!("Failed check role data: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed check role data".to_string()
            ))
        },
    }
}
