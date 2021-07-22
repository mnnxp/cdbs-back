use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Notification {
    pub id: i32,
    pub notification: String,
    pub id_degree_importance: i32,
    pub generated_at: NaiveDateTime,
    pub is_read: bool,
}

#[Object]
impl Notification {
    async fn id(&self) -> i32 {
        self.id.into()
    }
    async fn notification(&self) -> String {
        self.notification.clone()
    }
    async fn id_degree_importance(&self) -> i32 {
        self.id_degree_importance.into()
    }
    async fn generated_at(&self) -> NaiveDateTime {
        self.generated_at.into()
    }
    async fn is_read(&self) -> bool {
        self.is_read.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "notification_ref"]
pub struct InsertableNotification {
    pub notification: String,
    pub id_degree_importance: i32,
    pub generated_at: NaiveDateTime,
    pub is_read: bool,
}

#[derive(Debug, Deserialize)]
pub struct NotificationData {
    pub notification: String,
    pub id_degree_importance: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimNotification {
    pub notification: String,
    pub id_degree_importance: i32,
    pub is_read: bool,
}


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct NotificationToUser {
    pub id: i32,
    pub id_notification: i32,
    pub uuid_user: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "notification_to_user"]
pub struct InsertableNotificationToUser {
    pub id_notification: i32,
    pub uuid_user: Uuid,
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
