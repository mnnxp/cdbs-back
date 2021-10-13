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
    let check_fav = diesel::update(user_fav)
        .filter(user_favorite_uuid.eq(&data.user_favorite_uuid)
        .and(user_follower_uuid.eq(&data.user_follower_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite user
        .execute(conn)
        .expect("Failed check fav data");

    match check_fav {
        1_usize => Ok(true), // <-- turned flag to false
        0_usize => {
            // user not found in favorite list
            Err(ServiceError::BadRequest(
                "No data found".to_string()
            ))
        },
        _ => Err(ServiceError::InternalServerError),
    }
}
