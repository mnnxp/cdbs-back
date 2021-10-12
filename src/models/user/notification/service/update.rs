use crate::errors::ServiceResult;
use diesel::prelude::*;
use uuid::Uuid;

/// Update is_read is true for target notification
pub(crate) fn notification_is_read(
    logged_user_uuid: &Uuid,
    notification_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::notification_ref::dsl as notification_ref;
    use crate::schema::notification_to_user::dsl as notification_to_user;

    // check notification for logged user
    let notification_id = notification_to_user::notification_to_user
        .filter(notification_to_user::notification_id.eq(notification_id)
        .and(notification_to_user::user_uuid.eq(logged_user_uuid)))
        .select(notification_to_user::notification_id)
        .first::<i32>(conn)
        .expect("Failed check notification data");

    // change is_read for notification
    let res_change = diesel::update(notification_ref::notification_ref
        .filter(notification_ref::id.eq(notification_id)))
        .set(notification_ref::is_read.eq(true))
        .execute(conn)
        .expect("Failed set notification as read");

    Ok(matches!(res_change, 1_usize))
}
