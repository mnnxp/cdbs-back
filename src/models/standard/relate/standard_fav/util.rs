use crate::errors::ServiceResult;
use crate::schema::standard_fav::dsl as standard_fav;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking the subscribe for the standard
pub(crate) fn check_subscriber_by_uuid(
    target_standard_uuid: &Uuid,
    target_user_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let check_subscriber = standard_fav::standard_fav
        .filter(standard_fav::standard_uuid.eq(target_standard_uuid)
        .and(standard_fav::user_uuid.eq(target_user_uuid)))
        .execute(conn)
        .expect("Fail load uuid list target user");

    match check_subscriber {
        0 => Ok(false),
        _ => Ok(true),
    }
}
