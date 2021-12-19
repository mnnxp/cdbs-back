use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::access::util::check_access_user_for_user;
use crate::models::user::user_fav::model::{
    IptUserFavData, InsertableUserFav
};
use crate::models::user::notification::{
    model::{NotificationType, NotificationData},
    service::register::create_notification,
};
use crate::schema::user_fav::dsl as user_fav;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_user_fav(
    logged_user_uuid: &Uuid,
    user_favorite_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for user
    check_access_user_for_user(
        logged_user_uuid, // <-- logged user_uuid
        user_favorite_uuid, // <-- target user_uuid
        &need_access_level,
        conn
    )?;

    // check active following
    let check_fav = user_fav::user_fav
        .filter(user_fav::user_favorite_uuid.eq(&user_favorite_uuid)
        .and(user_fav::user_follower_uuid.eq(&logged_user_uuid)))
        .select(user_fav::is_enabled)
        .limit(1)
        .load(conn)
        .map_err(|err| {
            debug!("Failed check fav standard: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match check_fav.first() {
        Some(true) => Ok(false), // <-- if data already has
        Some(false) => {
            // if have need row, just update is_enabled to true
            diesel::update(user_fav::user_fav)
                .filter(user_fav::user_favorite_uuid.eq(&user_favorite_uuid)
                .and(user_fav::user_follower_uuid.eq(&logged_user_uuid)))
                .set(user_fav::is_enabled.eq(true))
                .returning(user_fav::is_enabled)
                .get_result::<bool>(conn)
                .map_err(|err| {
                    debug!("Failed add fav user: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
        None => {
            let data = IptUserFavData {
                user_favorite_uuid: *user_favorite_uuid,
                user_follower_uuid: *logged_user_uuid,
            };

            // add flag and date created
            let insertable_fav: InsertableUserFav = data.into();

            diesel::insert_into(user_fav::user_fav)
                .values(insertable_fav)
                .returning(user_fav::is_enabled)
                .get_result::<bool>(conn)
                .map_err(|err| {
                    debug!("Failed add fav user: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            // add notification for user
            create_notification(
                user_favorite_uuid,
                &NotificationData {
                    notification: "New follower".to_string(),
                    degree_importance: NotificationType::Info,
                },
                conn,
            )
        },
    }
}
