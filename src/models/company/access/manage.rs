use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::model::ChangeTypeAccessCompany;
use crate::models::company::access::util::check_is_owner_with_err;
use diesel::prelude::*;
use uuid::Uuid;

/// Change company type_access
pub(crate) fn change_company_type_access(
    logged_user_uuid: &Uuid,
    data: &ChangeTypeAccessCompany,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::company_ref::dsl::*;

    check_is_owner_with_err(
        logged_user_uuid,
        &data.company_uuid,
        conn
    )?;

    let change_access = diesel::update(company_ref
        .filter(uuid.eq(&data.company_uuid)
        .and(user_uuid.eq(logged_user_uuid))
        .and(type_access_id.ne(&data.new_type_access_id))))
        .set(type_access_id.eq(&data.new_type_access_id))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed get represent types: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(change_access > 0)
}
