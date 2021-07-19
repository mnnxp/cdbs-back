use crate::errors::ServiceResult;
use crate::models::user::notification::model::{
    Notification, NotificationData, InsertableNotification, SlimNotification,
};
use diesel::prelude::*;

pub fn create_notification(
    notification_data: NotificationData,
    conn: &PgConnection,
) -> ServiceResult<SlimNotification> {
    use crate::schema::notification_ref::dsl::notification_ref;

    let notification: InsertableNotification = notification_data.into();
    let inserted_notification: Notification = diesel::insert_into(notification_ref)
        .values(&notification)
        .get_result(conn)?;
    Ok(inserted_notification.into())
}
