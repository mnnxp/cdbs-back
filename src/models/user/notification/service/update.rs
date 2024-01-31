use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

/// Устанавливает уведомление как прочитанное.
pub(crate) fn set_notifications_as_read(
    logged_user_uuid: &Uuid,
    notifications_ids: &[i32],
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    // use crate::schema::notification_ref::dsl as notification_ref;
    use crate::schema::notification_to_user::dsl as notification_to_user;

    // check notification for logged user
    let res_change = diesel::update(notification_to_user::notification_to_user)
        .filter(notification_to_user::notification_id.eq_any(notifications_ids)
        .and(notification_to_user::user_uuid.eq(logged_user_uuid)))
        .set(notification_to_user::is_read.eq(true))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check notification data: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(res_change)
}
