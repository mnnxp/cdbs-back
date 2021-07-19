use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::notification::model::Notification;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn delete_notification(
    input_uuid_user: Uuid,
    id_notification_delete: i32,
    conn: &PgConnection,
) -> ServiceResult<Notification> {
    use crate::schema::notification_ref::dsl::*;
    use crate::schema::notification_ref::dsl::id as notification_ref_id;
    use crate::schema::notification_to_user::dsl::*;

    // debug!("fn input_uuid_user = {}", &input_uuid_user);
    // debug!("fn id_notification_delete = {}", &id_notification_delete);

    // find notification and check privileges for delete
    let find_notification: i32 = notification_to_user
        .filter(uuid_user.eq(input_uuid_user))
        .filter(id_notification.eq(id_notification_delete))
        .select(id_notification)
        .first(conn)
        .unwrap_or(0);

    match find_notification {
        1..=i32::MAX => {
            // delete notification and save delete data for send response
            let delete_notification: Notification =
                diesel::delete(notification_ref.filter(notification_ref_id.eq(find_notification)))
                    .get_result(conn)?;
            // debug!("fn delete_notification ={:?}", &delete_notification);
            Ok(delete_notification)
        }
        _ => Err(ServiceError::BadRequest(
            "The notification not you or not found.".to_string(),
        )),
    }
}
