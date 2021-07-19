use crate::schema::*;
use chrono::*;
// use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct Notification {
    pub id: i32,
    pub notification: String,
    pub id_degree_importance: i32,
    pub generated_at: NaiveDateTime,
    pub is_read: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "notification_ref"]
pub struct InsertableNotification {
    pub notification: String,
    pub id_degree_importance: i32,
    pub generated_at: NaiveDateTime,
    pub is_read: bool,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct NotificationData {
    pub notification: String,
    pub id_degree_importance: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimNotification {
    pub notification: String,
    pub id_degree_importance: i32,
    pub is_read: bool,
}

impl From<NotificationData> for InsertableNotification {
    fn from(notification_data: NotificationData) -> Self {
        let NotificationData {
            notification,
            id_degree_importance,
            ..
        } = notification_data;

        Self {
            notification,
            id_degree_importance,
            generated_at: chrono::Local::now().naive_local(),
            is_read: false,
        }
    }
}

impl From<Notification> for SlimNotification {
    fn from(notification: Notification) -> Self {
        let Notification {
            notification,
            id_degree_importance,
            is_read,
            ..
        } = notification;

        Self {
            notification,
            id_degree_importance,
            is_read,
        }
    }
}
