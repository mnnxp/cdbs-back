use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Notification {
    pub id: i32,
    pub notification: String,
    pub degree_importance_id: i32,
    pub created_at: NaiveDateTime,
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
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
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
    pub created_at: NaiveDateTime,
    pub is_read: bool,
}

#[derive(Debug, Clone)]
pub struct NotificationData {
    pub notification: String,
    pub degree_importance: NotificationType,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Critical,
    Error,
    Warning,
    Success,
    Info,
}

impl NotificationType {
    pub(crate) fn get_id(&self) -> i32 {
        match self {
            NotificationType::Critical => 1,
            NotificationType::Error => 2,
            NotificationType::Warning => 3,
            NotificationType::Success => 4,
            NotificationType::Info => 5,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ShowNotification {
    pub notification: String,
    pub degree_importance_id: i32,
    pub is_read: bool,
}

impl From<&Notification> for ShowNotification {
    fn from(notification: &Notification) -> Self {
        let Notification {
            notification,
            degree_importance_id,
            is_read,
            ..
        } = notification;

        Self {
            notification: notification.to_string(),
            degree_importance_id: *degree_importance_id,
            is_read: *is_read,
        }
    }
}

#[derive(Debug, Queryable)]
pub struct NotificationToUser {
    pub notification_id: i32,
    pub user_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "notification_to_user"]
pub struct InsertableNotificationToUser {
    pub notification_id: i32,
    pub user_uuid: Uuid,
}

impl From<&NotificationData> for InsertableNotification {
    fn from(notification_data: &NotificationData) -> Self {
        let NotificationData {
            notification,
            degree_importance,
            ..
        } = notification_data;

        Self {
            notification: notification.to_string(),
            degree_importance_id: degree_importance.get_id(),
            created_at: chrono::Local::now().naive_local(),
            is_read: false,
        }
    }
}
