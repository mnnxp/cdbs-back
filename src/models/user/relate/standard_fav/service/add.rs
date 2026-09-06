use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::notification::{
    model::{NotificationData, NotificationType},
    service::register::create_notification,
};
use crate::models::user::standard_fav::model::{InsertableStandardFav, IptStandardFavData};
use crate::schema::standard_fav::dsl as standard_fav;
use crate::schema::standard_ref::dsl as standard_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет стандарт в список избранного авторизованного пользователя.
pub(crate) fn add_standard_fav(
    logged_user_uuid: &Uuid,
    standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Standard,
        standard_uuid,
        AccessOperation::Read,
        conn,
    )?;

    // if have need row, just update is_enabled to true
    let check_fav = standard_fav::standard_fav
        .filter(
            standard_fav::standard_uuid
                .eq(standard_uuid)
                .and(standard_fav::user_uuid.eq(logged_user_uuid)),
        )
        .select(standard_fav::is_enabled)
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
            diesel::update(standard_fav::standard_fav)
                .filter(
                    standard_fav::standard_uuid
                        .eq(standard_uuid)
                        .and(standard_fav::user_uuid.eq(logged_user_uuid)),
                )
                .set(standard_fav::is_enabled.eq(true))
                .returning(standard_fav::is_enabled)
                .get_result::<bool>(conn)
                .map_err(|err| {
                    debug!("Failed add fav standard: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
        None => {
            let data = IptStandardFavData {
                user_uuid: *logged_user_uuid,
                standard_uuid: *standard_uuid,
            };

            // add flag and date created
            let insertable_fav: InsertableStandardFav = data.into();

            diesel::insert_into(standard_fav::standard_fav)
                .values(insertable_fav)
                .returning(standard_fav::is_enabled)
                .get_result::<bool>(conn)
                .map_err(|err| {
                    debug!("Failed add fav standard: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            new_notification(standard_uuid, conn)
        }
    }
}

fn new_notification(object_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<bool> {
    let user_uuid = standard_ref::standard_ref
        .filter(standard_ref::uuid.eq(object_uuid))
        .select(standard_ref::user_uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get owner company: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // add notification for user
    create_notification(
        &user_uuid,
        &NotificationData {
            notification: "New follower of your standard".to_string(),
            degree_importance: NotificationType::Info,
        },
        conn,
    )
}
