use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::notification::model::{
    Notification, NotificationToUser, ShowNotification,
};
use crate::schema::notification_ref::dsl as notification_ref;
use crate::schema::notification_to_user::dsl as notification_to_user;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn get_notifications(
    logged_user_uuid: &Uuid,
    target_ids: &[i32],
    limit: &i64,
    offset: &i64,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowNotification>> {
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
            get_by_ids(
                logged_user_uuid,
                target_ids,
                limit,
                offset,
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
) -> ServiceResult<Vec<ShowNotification>> {
    let get_list = notification_to_user::notification_to_user
        .filter(notification_to_user::user_uuid.eq(logged_user_uuid))
        .limit(*limit)
        .offset(*offset)
        .load::<NotificationToUser>(conn)
        .map_err(|err| {
            debug!("Failed get notifications: {:?}", err);
            ServiceError::InternalServerError
        })?;

    let mut notifications_ids_list: Vec<i32> = Vec::new();
    for value in &get_list {
        notifications_ids_list.push(value.notification_id)
    }

    let get_data_list = notification_ref::notification_ref
        .filter(notification_ref::id.eq_any(&notifications_ids_list))
        .load::<Notification>(conn)
        .map_err(|err| {
            debug!("Failed get data notifications: {:?}", err);
            ServiceError::InternalServerError
        })?;

    let mut res: Vec<ShowNotification> = Vec::new();
    for x in get_list.iter() {
        for y in get_data_list.iter() {
            if x.notification_id == y.id {
                res.push((y, x).into());
                break;
            }
        }
    }

    Ok(res)
}

/// Gets notification for target user by ids list
fn get_by_ids(
    logged_user_uuid: &Uuid,
    target_ids: &[i32],
    limit: &i64,
    offset: &i64,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowNotification>> {
    let get_list = notification_to_user::notification_to_user
        .filter(notification_to_user::user_uuid.eq(logged_user_uuid)
        .and(notification_to_user::notification_id.eq_any(target_ids)))
        .limit(*limit)
        .offset(*offset)
        .load::<NotificationToUser>(conn)
        .map_err(|err| {
            debug!("Failed get notifications: {:?}", err);
            ServiceError::InternalServerError
        })?;

    let mut notifications_ids_list: Vec<i32> = Vec::new();
    for value in &get_list {
        notifications_ids_list.push(value.notification_id)
    }

    let get_data_list = notification_ref::notification_ref
        .filter(notification_ref::id.eq_any(&notifications_ids_list))
        .load::<Notification>(conn)
        .map_err(|err| {
            debug!("Failed get data notifications: {:?}", err);
            ServiceError::InternalServerError
        })?;

    let mut res: Vec<ShowNotification> = Vec::new();
    for x in get_list.iter() {
        for y in get_data_list.iter() {
            if x.notification_id == y.id {
                res.push((y, x).into());
                break;
            }
        }
    }

    Ok(res)
}
