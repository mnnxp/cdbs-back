use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::access::util::check_access_user_for_user;
use crate::models::user::user_fav::model::{
    IptUserFavData, InsertableUserFav
};
use crate::schema::user_fav::dsl::*;
use diesel::prelude::*;

pub(crate) fn add_user_fav(
    data: &IptUserFavData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for user
    check_access_user_for_user(
        &data.user_favorite_uuid,
        &data.user_favorite_uuid,
        &need_access_level,
        conn
    )?;

    // if have need row, just update is_enabled to true
    let check_fav = diesel::update(user_fav)
        .filter(user_favorite_uuid.eq(&data.user_favorite_uuid)
        .and(user_follower_uuid.eq(&data.user_follower_uuid)))
        .set(is_enabled.eq(true))
        .execute(conn)
        .expect("Failed check fav data");

    match check_fav {
        1_usize => Ok(true),
        0_usize => {
            // add flag and date created
            let insertable_fav: InsertableUserFav = data.into();

            diesel::insert_into(user_fav)
                .values(insertable_fav)
                .execute(conn)
                .expect("Failed add fav data");

            Ok(true)
        },
        _ => Err(ServiceError::InternalServerError),
    }
}
