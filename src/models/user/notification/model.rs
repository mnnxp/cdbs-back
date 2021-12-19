use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Associations, Queryable, Clone, Debug)]
#[primary_key(id)]
#[table_name = "notification_ref"]
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

#[derive(Debug, Serialize, Clone, SimpleObject)]
pub struct ShowNotification {
    pub id: i32,
    pub notification: String,
    pub degree_importance: DegreeImportanceTranslateList,
    pub created_at: NaiveDateTime,
    pub is_read: bool,
}

impl ShowNotification {
    /// Create struct with Notification data, related data set default
    pub(crate) fn new(data: &Notification) -> Self {
        Self{
            id: data.id,
            notification: data.notification.clone(),
            degree_importance: Default::default(),
            created_at: data.created_at,
            is_read: false,
        }
    }

    /// Change is_read in norification data
    pub(crate) fn put_is_read(&mut self, is_read: &bool) {
        self.is_read = *is_read;
    }

    /// Change degree_importance data
    pub(crate) fn put_degree_importance(&mut self, degree_importance: &DegreeImportanceTranslateList) {
        self.degree_importance = degree_importance.clone();
    }
}

#[derive(Debug, Queryable, Default)]
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

#[derive(Identifiable, Serialize, Associations, Queryable)]
#[derive(Default, Clone, Debug, SimpleObject)]
#[primary_key(degree_importance_id, lang_id)]
#[table_name = "degree_importance_translate_list"]
pub struct DegreeImportanceTranslateList {
    pub degree_importance_id: i32,
    pub lang_id: i32,
    pub degree: String,
}
