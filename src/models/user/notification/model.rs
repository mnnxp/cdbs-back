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

impl From<(&Notification, &NotificationToUser, &DegreeImportanceTranslateList)> for ShowNotification {
    fn from(data: (&Notification, &NotificationToUser, &DegreeImportanceTranslateList)) -> Self {
        if data.1.notification_id != data.0.id {
            debug!("Filed data.1.notification_id and data.0.id: {:?} != {:?}",
                &data.1.notification_id,
                &data.0.id)
        }

        if data.0.degree_importance_id != data.2.degree_importance_id {
            debug!("Filed data.0 and data.2: {:?} != {:?}",
                &data.0.degree_importance_id,
                &data.2.degree_importance_id)
        }

        Self {
            id: data.0.id,
            notification: data.0.notification.to_string(),
            degree_importance: DegreeImportanceTranslateList {
                degree_importance_id: data.2.degree_importance_id,
                lang_id: data.2.lang_id,
                degree: data.2.degree.to_string(),
            },
            created_at: data.0.created_at,
            is_read: data.1.is_read,
        }
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

#[derive(Identifiable, Serialize, Associations, Queryable, Default, Clone, Debug, SimpleObject)]
#[primary_key(degree_importance_id, lang_id)]
#[table_name = "degree_importance_translate_list"]
pub struct DegreeImportanceTranslateList {
    pub degree_importance_id: i32,
    pub lang_id: i32,
    pub degree: String,
}
