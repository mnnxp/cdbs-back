use crate::errors::{ServiceResult, ServiceError};
use crate::schema::user_fav::dsl as user_fav;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking the subscribe for the user
pub(crate) fn check_subscriber_by_uuid(
    target_user_uuid: &Uuid,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let check_subscriber = user_fav::user_fav
        .filter(user_fav::user_favorite_uuid.eq(target_user_uuid)
        .and(user_fav::user_follower_uuid.eq(logged_user_uuid))
        .and(user_fav::is_enabled.eq(true)))
        .execute(conn)
        .map_err(|err| {
            debug!("Fail load uuid list target user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(check_subscriber > 0)
}
