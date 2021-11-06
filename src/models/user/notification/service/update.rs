use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

/// Update is_read is true for target notification
pub(crate) fn notification_is_read(
    logged_user_uuid: &Uuid,
    notification_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // use crate::schema::notification_ref::dsl as notification_ref;
    use crate::schema::notification_to_user::dsl as notification_to_user;

    // check notification for logged user
    let res_change = diesel::update(notification_to_user::notification_to_user)
        .filter(notification_to_user::notification_id.eq(notification_id)
        .and(notification_to_user::user_uuid.eq(logged_user_uuid)))
        .set(notification_to_user::is_read.eq(true))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check notification data: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(matches!(res_change, 1_usize))
}
