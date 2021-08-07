use crate::database::{get_conn, PooledConnection};
use crate::errors::{ServiceError, ServiceResult};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::user::notification::model::Notification;
use diesel::prelude::*;
// use std::any::Any;
use uuid::Uuid;

pub(crate) fn get_notifications(
    context: &Context<'_>,
    id_notification_search: i32,
    target_uuid_user: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Notification>> {
    let mut variant_selection: u8 = 0;
    if id_notification_search > 0 {
        variant_selection += 1;
    }
    // if target_uuid_user > Uuid::nil() {
    //     variant_selection += 10;
    // }

    match variant_selection {
        0 => find_all_notifications(context, target_uuid_user, limit, offset),
        1 => find_id_notification(context, id_notification_search, target_uuid_user),
        // 10
        // 11
        // 100
        // 101
        // 110
        // 111
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_notifications(
    context: &Context<'_>,
    target_uuid_user: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Notification>> {
    use crate::schema::notification_ref::dsl::*;
    use crate::schema::notification_ref::dsl::id as notification_ref_id;
    use crate::schema::notification_to_user::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(notification_ref
        .inner_join(notification_to_user)
        .filter(uuid_user.eq(target_uuid_user))
        .select((
            notification_ref_id, notification, id_degree_importance,
            generated_at, is_read,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Notification>(conn)?)
}

fn find_id_notification(
    context: &Context<'_>,
    id_notification_search: i32,
    target_uuid_user: Uuid,
) -> ServiceResult<Vec<Notification>> {
    use crate::schema::notification_ref::dsl::*;
    use crate::schema::notification_ref::dsl::id as notification_ref_id;
    use crate::schema::notification_to_user::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(notification_ref
        .inner_join(notification_to_user)
        .filter(uuid_user.eq(target_uuid_user))
        .filter(id_notification.eq(id_notification_search))
        .select((
            notification_ref_id, notification, id_degree_importance,
            generated_at, is_read,
        ))
        .load::<Notification>(conn)?)
}
