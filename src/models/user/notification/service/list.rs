use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use async_graphql::Context;
use crate::models::user::notification::model::Notification;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn get_notifications(
    cxt: &Context<'_>,
    search_ids: &[i32],
    target_user_uuid: &Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Notification>> {
    if search_ids.is_empty() {
        get_all(cxt, target_user_uuid, limit, offset)
    } else {
        get_by_id(cxt, search_ids, target_user_uuid)
    }
}

/// Gets all notification for target user
fn get_all(
    cxt: &Context<'_>,
    target_user_uuid: &Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Notification>> {
    use crate::schema::notification_ref::dsl::*;
    use crate::schema::notification_ref::dsl::id as notification_ref_id;
    use crate::schema::notification_to_user::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(notification_ref
        .inner_join(notification_to_user)
        .filter(user_uuid.eq(target_user_uuid))
        .select((
            notification_ref_id, notification, degree_importance_id,
            generated_at, is_read,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Notification>(conn)?)
}

/// Gets notification for target user by id list
fn get_by_id(
    cxt: &Context<'_>,
    search_ids: &[i32],
    target_user_uuid: &Uuid,
) -> ServiceResult<Vec<Notification>> {
    use crate::schema::notification_ref::dsl::*;
    use crate::schema::notification_ref::dsl::id as notification_ref_id;
    use crate::schema::notification_to_user::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(notification_ref
        .inner_join(notification_to_user)
        .filter(user_uuid.eq(target_user_uuid)
        .and(notification_id.eq_any(search_ids)))
        .select((
            notification_ref_id, notification, degree_importance_id,
            generated_at, is_read,
        ))
        .load::<Notification>(conn)?)
}
