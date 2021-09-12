use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Notification {
    pub id: i32,
    pub notification: String,
    pub degree_importance_id: i32,
    pub generated_at: NaiveDateTime,
    pub is_read: bool,
}

#[Object]
impl Notification {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn notification(&self) -> &String {
        &self.notification
    }
    async fn degree_importance_id(&self) -> &i32 {
        &self.degree_importance_id
    }
    async fn generated_at(&self) -> &NaiveDateTime {
        &self.generated_at
    }
    async fn is_read(&self) -> &bool {
        &self.is_read
    }
}

#[derive(Debug, Insertable)]
#[table_name = "notification_ref"]
pub struct InsertableNotification {
    pub notification: String,
    pub degree_importance_id: i32,
    pub generated_at: NaiveDateTime,
    pub is_read: bool,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct NotificationData {
    pub notification: String,
    pub degree_importance_id: i32,
}

// #[Object]
// impl NotificationData {
//     async fn notification(&self) -> &String {
//         &self.notification
//     }
//     async fn degree_importance_id(&self) -> &i32 {
//         &self.degree_importance_id
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimNotification {
    pub notification: String,
    pub degree_importance_id: i32,
    pub is_read: bool,
}

#[Object]
impl SlimNotification {
    async fn notification(&self) -> &String {
        &self.notification
    }
    async fn degree_importance_id(&self) -> &i32 {
        &self.degree_importance_id
    }
    async fn is_read(&self) -> &bool {
        &self.is_read
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct NotificationToUser {
    pub id: i32,
    pub notification_id: i32,
    pub user_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "notification_to_user"]
pub struct InsertableNotificationToUser {
    pub notification_id: i32,
    pub user_uuid: Uuid,
}

impl From<NotificationData> for InsertableNotification {
    fn from(notification_data: NotificationData) -> Self {
        let NotificationData {
            notification,
            degree_importance_id,
            ..
        } = notification_data;

        Self {
            notification,
            degree_importance_id,
            generated_at: chrono::Local::now().naive_local(),
            is_read: false,
        }
    }
}

impl From<Notification> for SlimNotification {
    fn from(notification: Notification) -> Self {
        let Notification {
            notification,
            degree_importance_id,
            is_read,
            ..
        } = notification;

        Self {
            notification,
            degree_importance_id,
            is_read,
        }
    }
}
