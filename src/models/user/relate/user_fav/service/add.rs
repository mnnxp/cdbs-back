use crate::errors::{
    // ServiceError,
    ServiceResult
};
use crate::database::PooledConnection;
use crate::models::user::user_fav::model::{
    UserFav,
    IptUserFavData,
    InsertableUserFav,
};
use crate::schema::user_fav::dsl::*;
use diesel::prelude::*;

pub(crate) fn add_user_fav(
    data: IptUserFavData,
    conn: &PooledConnection,
) -> ServiceResult<UserFav> {
    // if have need row, just update is_enabled to true
    let check_fav = diesel::update(user_fav)
        .filter(user_favorite_uuid.eq(&data.user_favorite_uuid)
        .and(user_follower_uuid.eq(&data.user_follower_uuid)))
        .set(is_enabled.eq(true))
        .get_result(conn);

    let user_user_fav: UserFav = match check_fav {
        Ok(fav) => fav,
        Err(_) => {
            // add flag and date created
            let insertable_fav: InsertableUserFav = data.into();

            diesel::insert_into(user_fav)
                .values(insertable_fav)
                .get_result(conn)?
        },
    };

    debug!("User favorite user: {:#?}", user_user_fav);

    Ok(user_user_fav)
}
