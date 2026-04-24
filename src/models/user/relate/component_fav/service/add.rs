use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::component_fav::model::{InsertableComponentFav, IptComponentFavData};
use crate::models::user::notification::{
    model::{NotificationData, NotificationType},
    service::register::create_notification,
};
use crate::schema::component_fav::dsl as component_fav;
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет компонент в список избранного авторизованного пользователя.
pub(crate) fn add_component_fav(
    logged_user_uuid: &Uuid,
    component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        component_uuid,
        AccessOperation::Read,
        conn,
    )?;

    // if have need row, just update is_enabled to true
    let check_fav = component_fav::component_fav
        .filter(
            component_fav::component_uuid
                .eq(component_uuid)
                .and(component_fav::user_uuid.eq(logged_user_uuid)),
        )
        .select(component_fav::is_enabled)
        .limit(1)
        .load(conn)
        .map_err(|err| {
            debug!("Failed check fav component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match check_fav.first() {
        Some(true) => Ok(false), // <-- if data already has
        Some(false) => {
            // if have need row, just update is_enabled to true
            diesel::update(component_fav::component_fav)
                .filter(
                    component_fav::component_uuid
                        .eq(component_uuid)
                        .and(component_fav::user_uuid.eq(logged_user_uuid)),
                )
                .set(component_fav::is_enabled.eq(true))
                .returning(component_fav::is_enabled)
                .get_result::<bool>(conn)
                .map_err(|err| {
                    debug!("Failed add fav component: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
        None => {
            // creating a new record for tracking the component by the user
            component_to_fav_ft(logged_user_uuid, component_uuid, conn)?;
            new_notification(component_uuid, conn)
        }
    }
}

/// Добавление новой записи для добавления компонента в избранное пользователя
/// Adding a new entry to add a component to a user's favorites for first time
pub(crate) fn component_to_fav_ft(
    user_uuid: &Uuid,
    component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let data = IptComponentFavData {
        user_uuid: *user_uuid,
        component_uuid: *component_uuid,
    };

    // add flag and date created
    let insertable_fav: InsertableComponentFav = data.into();

    diesel::insert_into(component_fav::component_fav)
        .values(insertable_fav)
        .returning(component_fav::is_enabled)
        .get_result::<bool>(conn)
        .map_err(|err| {
            debug!("Failed add fav component: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn new_notification(object_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<bool> {
    let user_uuid = component_ref::component_ref
        .filter(component_ref::uuid.eq(object_uuid))
        .select(component_ref::user_uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get owner company: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // add notification for user
    create_notification(
        &user_uuid,
        &NotificationData {
            notification: "New follower of your component".to_string(),
            degree_importance: NotificationType::Info,
        },
        conn,
    )
}
