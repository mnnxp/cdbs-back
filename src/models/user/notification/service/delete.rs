use crate::errors::ServiceResult;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete notifications logged user
pub(crate) fn delete_notifications(
    logged_user_uuid: &Uuid,
    notifications_ids: &[i32],
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    use crate::schema::notification_ref::dsl as notification_ref;
    use crate::schema::notification_to_user::dsl as notification_to_user;

    // find notification and check privileges for delete
    let find_notifications = notification_to_user::notification_to_user
        .filter(notification_to_user::user_uuid.eq(logged_user_uuid)
        .and(notification_to_user::notification_id.eq_any(notifications_ids)))
        .select(notification_to_user::notification_id)
        .load::<i32>(conn)
        .expect("Not found notification");

    // delete notification and save delete data for send response
    let count_del = diesel::delete(notification_ref::notification_ref
        .filter(notification_ref::id.eq_any(find_notifications)))
        .execute(conn)
        .expect("Failed delete notification");

    // debug!("fn delete_notification ={:?}", &delete_notification);

    Ok(count_del as i32)
}
