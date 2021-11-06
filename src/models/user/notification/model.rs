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
    pub id: i32,
    pub notification: String,
    pub degree_importance_id: i32,
    pub created_at: NaiveDateTime,
    pub is_read: bool,
}

#[Object]
impl ShowNotification {
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

impl From<(&Notification, &NotificationToUser)> for ShowNotification {
    fn from(data: (&Notification, &NotificationToUser)) -> Self {
        let is_read = match data.1.notification_id == data.0.id {
            true => data.1.is_read,
            false => false,
        };

        Self {
            id: data.0.id,
            notification: data.0.notification.to_string(),
            degree_importance_id: data.0.degree_importance_id,
            created_at: data.0.created_at,
            is_read
        }
    }
}

#[derive(Debug, Queryable)]
pub struct NotificationToUser {
    pub notification_id: i32,
    pub user_uuid: Uuid,
    pub is_read: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "notification_to_user"]
pub struct InsertableNotificationToUser {
    pub notification_id: i32,
    pub user_uuid: Uuid,
    pub is_read: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "notification_ref"]
pub struct InsertableNotification {
    pub notification: String,
    pub degree_importance_id: i32,
    pub created_at: NaiveDateTime,
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
        }
    }
}
