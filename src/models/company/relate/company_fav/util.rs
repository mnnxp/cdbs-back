use crate::errors::{ServiceResult, ServiceError};
use crate::schema::company_fav::dsl as company_fav;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking the subscribe for the company
pub(crate) fn check_subscriber_by_uuid(
    target_company_uuid: &Uuid,
    target_user_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let check_subscriber = company_fav::company_fav
        .filter(company_fav::company_uuid.eq(target_company_uuid)
        .and(company_fav::user_uuid.eq(target_user_uuid))
        .and(company_fav::is_enabled.eq(true)))
        .execute(conn)
        .map_err(|err| {
            debug!("Fail load uuid list target user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(check_subscriber > 0)
}
