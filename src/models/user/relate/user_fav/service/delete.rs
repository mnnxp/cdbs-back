use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::user::user_fav::model::{
    UserFav,
    IptUserFavData,
};
use crate::schema::user_fav::dsl::*;
use diesel::prelude::*;

// Remove a user from user favorites user list
pub(crate) fn delete_user_fav(
    data: IptUserFavData,
    conn: &PgConnection,
) -> ServiceResult<UserFav> {
    // if have need row, just update is_enabled to false
    let check_fav = diesel::update(user_fav)
        .filter(user_favorite_uuid.eq(&data.user_favorite_uuid)
        .and(user_follower_uuid.eq(&data.user_follower_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite user
        .get_result(conn);

    let user_user_fav: UserFav = match check_fav {
        Ok(fav) => fav, // <-- turned flag to false
        Err(err) => {
            debug!("Err with delete user fav: {:#?}", err);
            // user not found in favorite list
            return Err(ServiceError::BadRequest("User not found in favotite list".to_string()))
        },
    };

    debug!("User delete from favorute: {:#?}", user_user_fav);

    Ok(user_user_fav)
}
