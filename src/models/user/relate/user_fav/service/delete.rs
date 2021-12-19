use crate::errors::{ServiceError, ServiceResult};
use crate::schema::user_fav::dsl as user_fav;
use diesel::prelude::*;
use uuid::Uuid;

// Remove a user from user favorites user list
pub(crate) fn delete_user_fav(
    logged_user_uuid: &Uuid,
    user_favorite_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // if have need row, just update is_enabled to false
    let del_fav = diesel::update(user_fav::user_fav)
        .filter(user_fav::user_favorite_uuid.eq(user_favorite_uuid)
        .and(user_fav::user_follower_uuid.eq(logged_user_uuid))
        .and(user_fav::is_enabled.eq(true))) // <-- active favorite
        .set(user_fav::is_enabled.eq(false)) // <-- off favorite user
        .returning(user_fav::is_enabled)
        .get_result::<bool>(conn)
        .map_err(|err| {
            debug!("Failed delete fav user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(!del_fav)
}
