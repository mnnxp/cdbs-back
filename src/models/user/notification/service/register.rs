use crate::errors::ServiceResult;
use crate::models::user::notification::model::{
    Notification, NotificationData, InsertableNotification, SlimNotification,
    NotificationToUser, InsertableNotificationToUser,
};
use diesel::prelude::*;
use uuid::Uuid;

pub fn create_notification(
    notification_data: NotificationData,
    user_uuid: Uuid,
    conn: &PgConnection,
) -> ServiceResult<SlimNotification> {
    use crate::schema::notification_ref::dsl::notification_ref;
    use crate::schema::notification_to_user::dsl::notification_to_user;

    // debug!("fn notification_id_delete = {}", &notification_id_delete);

    let notification: InsertableNotification = notification_data.into();
    let inserted_notification: Notification = diesel::insert_into(notification_ref)
        .values(&notification)
        .get_result(conn)?;
    // debug!("fn input_user_uuid = {}", &input_user_uuid);

    // add row to notification_to_user with current user
    let row_notification_to_user: InsertableNotificationToUser = InsertableNotificationToUser{
        notification_id: (inserted_notification.id),
        user_uuid: (user_uuid),
    };
    let row_notification_to_user: NotificationToUser = diesel::insert_into(notification_to_user)
        .values(&row_notification_to_user)
        .get_result(conn)?;

    debug!("Entry added successfully = {:?}", row_notification_to_user);

    Ok(inserted_notification.into())
}
