use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Queryable, Clone, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = notification_ref)]
pub(crate) struct Notification {
    pub(crate) id: i32,
    pub(crate) notification: String,
    pub(crate) degree_importance_id: i32,
    pub(crate) created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub(crate) struct NotificationData {
    pub(crate) notification: String,
    pub(crate) degree_importance: NotificationType,
}

#[derive(Debug, Clone)]
pub(crate) enum NotificationType {
    // Critical,
    // Error,
    // Warning,
    Success,
    Info,
}

impl NotificationType {
    pub(crate) fn get_id(&self) -> i32 {
        match self {
            // NotificationType::Critical => 1,
            // NotificationType::Error => 2,
            // NotificationType::Warning => 3,
            NotificationType::Success => 4,
            NotificationType::Info => 5,
        }
    }
}

/// Данные об уведомлении пользователя
#[derive(Debug, Serialize, Clone, SimpleObject)]
pub(crate) struct ShowNotification {
    /// Идентификатор уведомления
    pub(crate) id: i32,
    /// Информация уведомления
    pub(crate) notification: String,
    /// Степень важности уведомления с локализацией
    pub(crate) degree_importance: DegreeImportanceTranslateList,
    /// Дата создания уведомления
    pub(crate) created_at: NaiveDateTime,
    /// Флаг прочтения уведомления
    pub(crate) is_read: bool,
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
pub(crate) struct NotificationToUser {
    pub(crate) notification_id: i32,
    // pub(crate) user_uuid: Uuid,
    pub(crate) is_read: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = notification_to_user)]
pub(crate) struct InsertableNotificationToUser {
    pub(crate) notification_id: i32,
    pub(crate) user_uuid: Uuid,
    pub(crate) is_read: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = notification_ref)]
pub(crate) struct InsertableNotification {
    pub(crate) notification: String,
    pub(crate) degree_importance_id: i32,
    pub(crate) created_at: NaiveDateTime,
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

/// Данные о степени (уровне) важности уведомления
#[derive(Identifiable, Serialize, Queryable)]
#[derive(Default, Clone, Debug, SimpleObject)]
#[diesel(primary_key(degree_importance_id, lang_id))]
#[diesel(table_name = degree_importance_translate_list)]
pub(crate) struct DegreeImportanceTranslateList {
    /// Идентификатор степени важности
    pub(crate) degree_importance_id: i32,
    /// Идентификатор языка локализации
    pub(crate) lang_id: i32,
    /// Текстовое описание степени важности
    pub(crate) degree: String,
}

/// Аргументы запроса на получение уведомлений пользователя
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptNotificationArg {
    /// Фильтр по идентификаторам степени важности уведомления
    pub(crate) notification_ids:  Option<Vec<i32>>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct NotificationArg {
    pub(crate) notification_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for NotificationArg {
    fn default() -> Self {
        Self {
            notification_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptNotificationArg> for NotificationArg {
    fn from(data: IptNotificationArg) -> Self {
        let IptNotificationArg {
            notification_ids,
            limit,
            offset,
        } = data;

        Self {
            notification_ids: notification_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
