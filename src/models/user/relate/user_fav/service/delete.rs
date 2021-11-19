use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::user_fav::model::IptUserFavData;
use crate::schema::user_fav::dsl::*;
use diesel::prelude::*;

// Remove a user from user favorites user list
pub(crate) fn delete_user_fav(
    data: &IptUserFavData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // if have need row, just update is_enabled to false
    let del_fav = diesel::update(user_fav)
        .filter(user_favorite_uuid.eq(&data.user_favorite_uuid)
        .and(user_follower_uuid.eq(&data.user_follower_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite user
        .returning(is_enabled)
        .get_result::<bool>(conn)
        .map_err(|err| {
            debug!("Failed delete fav user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(!del_fav)
}
