use crate::errors::ServiceResult;
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
        .and(company_fav::user_uuid.eq(target_user_uuid)))
        .execute(conn)
        .expect("Fail load uuid list target user");

    match check_subscriber {
        0 => Ok(false),
        _ => Ok(true),
    }
}
