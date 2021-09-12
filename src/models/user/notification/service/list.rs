use crate::database::{get_conn, PooledConnection};
use crate::errors::{ServiceError, ServiceResult};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::user::notification::model::Notification;
use diesel::prelude::*;
// use std::any::Any;
use uuid::Uuid;

pub(crate) fn get_notifications(
    cxt: &Context<'_>,
    notification_id_search: i32,
    target_user_uuid: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Notification>> {
    let mut variant_selection: u8 = 0;
    if notification_id_search > 0 {
        variant_selection += 1;
    }
    // if target_user_uuid > Uuid::nil() {
    //     variant_selection += 10;
    // }

    match variant_selection {
        0 => find_all_notifications(cxt, target_user_uuid, limit, offset),
        1 => find_notification_id(cxt, notification_id_search, target_user_uuid),
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
    cxt: &Context<'_>,
    target_user_uuid: Uuid,
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
            notification_ref_id, notification, id_degree_importance,
            generated_at, is_read,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Notification>(conn)?)
}

fn find_notification_id(
    cxt: &Context<'_>,
    notification_id_search: i32,
    target_user_uuid: Uuid,
) -> ServiceResult<Vec<Notification>> {
    use crate::schema::notification_ref::dsl::*;
    use crate::schema::notification_ref::dsl::id as notification_ref_id;
    use crate::schema::notification_to_user::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(notification_ref
        .inner_join(notification_to_user)
        .filter(user_uuid.eq(target_user_uuid))
        .filter(notification_id.eq(notification_id_search))
        .select((
            notification_ref_id, notification, id_degree_importance,
            generated_at, is_read,
        ))
        .load::<Notification>(conn)?)
}
