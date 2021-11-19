use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::access::util::check_access_user_for_user;
use crate::models::user::user_fav::model::{
    IptUserFavData, InsertableUserFav
};
use crate::schema::user_fav::dsl as user_fav;
use diesel::prelude::*;

pub(crate) fn add_user_fav(
    data: &IptUserFavData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for user
    check_access_user_for_user(
        &data.user_follower_uuid, // <-- logged user_uuid
        &data.user_favorite_uuid, // <-- target user_uuid
        &need_access_level,
        conn
    )?;

    // check active following
    let check_fav = user_fav::user_fav
        .filter(user_fav::user_favorite_uuid.eq(&data.user_favorite_uuid)
        .and(user_fav::user_follower_uuid.eq(&data.user_follower_uuid)))
        .select(user_fav::is_enabled)
        .first(conn);

    match check_fav {
        Ok(fav) => {
            if fav {
                // if data already has
                Ok(false)
            } else {
                // if have need row, just update is_enabled to true
                diesel::update(user_fav::user_fav)
                    .filter(user_fav::user_favorite_uuid.eq(&data.user_favorite_uuid)
                    .and(user_fav::user_follower_uuid.eq(&data.user_follower_uuid)))
                    .set(user_fav::is_enabled.eq(true))
                    .returning(user_fav::is_enabled)
                    .get_result(conn)
                    .map_err(|err| {
                        debug!("Failed add fav user: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
        },
        Err(err) => {
            debug!("Err check is_enabled: {:?}", err);

            // add flag and date created
            let insertable_fav: InsertableUserFav = data.into();

            diesel::insert_into(user_fav::user_fav)
                .values(insertable_fav)
                .returning(user_fav::is_enabled)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed add fav user: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
