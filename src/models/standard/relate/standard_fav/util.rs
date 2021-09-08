use crate::errors::ServiceResult;
use crate::schema::standard_fav::dsl as standard_fav;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking the subscribe for the standard
pub(crate) fn check_subscriber_by_uuid(
    target_uuid_standard: &Uuid,
    target_uuid_user: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let check_subscriber = standard_fav::standard_fav
        .filter(standard_fav::uuid_standard.eq(target_uuid_standard)
        .and(standard_fav::uuid_user.eq(target_uuid_user)))
        .execute(conn)
        .expect("Fail load uuid list target user");

    match check_subscriber {
        0 => Ok(false),
        _ => Ok(true),
    }
}
