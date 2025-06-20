use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::notification::model::{
    InsertableNotification, InsertableNotificationToUser, NotificationData,
};
use diesel::prelude::*;
use uuid::Uuid;

/// Add notification for target user
pub(crate) fn create_notification(
    target_user_uuid: &Uuid,
    notification_data: &NotificationData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::notification_ref::dsl as notification_ref;
    use crate::schema::notification_to_user::dsl as notification_to_user;

    let insert_notification_data: InsertableNotification = notification_data.into();

    let notification_id = diesel::insert_into(notification_ref::notification_ref)
        .values(&insert_notification_data)
        .returning(notification_ref::id)
        .get_result::<i32>(conn)
        .map_err(|err| {
            debug!("Failed insert notification: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // debug!("fn input_user_uuid = {}", &input_user_uuid);

    let row_notification_to_user: InsertableNotificationToUser = InsertableNotificationToUser {
        notification_id,
        user_uuid: *target_user_uuid,
        is_read: false,
    };

    // add row with notification id and target user
    diesel::insert_into(notification_to_user::notification_to_user)
        .values(&row_notification_to_user)
        .returning(notification_to_user::notification_id)
        .get_result::<i32>(conn)
        .map_err(|err| {
            debug!(
                "Failed insert notification data related with user: {:?}",
                err
            );
            ServiceError::InternalServerError
        })?;

    Ok(true)
}
