use crate::errors::ServiceResult;
use crate::schema::user_fav::dsl as user_fav;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking the subscribe for the user
pub(crate) fn check_subscriber_by_uuid(
    target_uuid_user: &Uuid,
    logged_uuid_user: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let check_subscriber = user_fav::user_fav
        .filter(user_fav::uuid_user_favorite.eq(target_uuid_user)
        .and(user_fav::uuid_user_follower.eq(logged_uuid_user)))
        .execute(conn)
        .expect("Fail load uuid list target user");

    match check_subscriber {
        0 => Ok(false),
        _ => Ok(true),
    }
}
