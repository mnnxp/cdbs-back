use crate::models::user::notification::model::Notification;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn get_notifications(
    logged_user_uuid: &Uuid,
    target_ids: &[i32],
    limit: &i64,
    offset: &i64,
    conn: &PgConnection,
) -> Vec<Notification> {
    match target_ids.is_empty() {
        true => {
            get_all(
                logged_user_uuid,
                limit,
                offset,
                conn
            )
        },
        false => {
            get_by_id(
                logged_user_uuid,
                target_ids,
                conn
            )
        },
    }
}

/// Gets all notification for target user
fn get_all(
    logged_user_uuid: &Uuid,
    limit: &i64,
    offset: &i64,
    conn: &PgConnection,
) -> Vec<Notification> {
    use crate::schema::notification_ref::dsl::*;
    use crate::schema::notification_ref::dsl::id as notification_ref_id;
    use crate::schema::notification_to_user::dsl::*;

    notification_ref
        .inner_join(notification_to_user)
        .filter(user_uuid.eq(logged_user_uuid))
        .select((
            notification_ref_id,
            notification,
            degree_importance_id,
            created_at,
            is_read,
        ))
        .limit(*limit)
        .offset(*offset)
        .load::<Notification>(conn)
        .expect("Failed get notifications")
}

/// Gets notification for target user by id list
fn get_by_id(
    logged_user_uuid: &Uuid,
    target_ids: &[i32],
    conn: &PgConnection,
) -> Vec<Notification> {
    use crate::schema::notification_ref::dsl::*;
    use crate::schema::notification_ref::dsl::id as notification_ref_id;
    use crate::schema::notification_to_user::dsl::*;

    notification_ref
        .inner_join(notification_to_user)
        .filter(user_uuid.eq(logged_user_uuid)
        .and(notification_id.eq_any(target_ids)))
        .select((
            notification_ref_id,
            notification,
            degree_importance_id,
            created_at,
            is_read,
        ))
        .load::<Notification>(conn)
        .expect("Failed get notifications")
}
