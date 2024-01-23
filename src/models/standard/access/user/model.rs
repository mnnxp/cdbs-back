use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::{InputObject, SimpleObject};
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(standard_uuid, user_uuid))]
#[diesel(table_name = user_access_to_standard)]
pub(crate) struct UserAccessStandard {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Данные о наличии доступа к стандарту у пользователя
#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct UserAccessStandardAndRelatedData {
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
    /// UUID пользователя
    pub(crate) user_uuid: Uuid,
    /// Тип доступа с локализацией
    pub(crate) type_access: TypeAccessTranslateList,
    /// Флаг активности доступа
    pub(crate) is_enabled: bool,
    /// Дата первой выдачи доступа пользователю
    pub(crate) created_at: NaiveDateTime,
    /// Дата обновления доступа
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_access_to_standard)]
pub(crate) struct InsertableUserAccessStandard {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Данные для запроса на выдачу пользователю доступа к стандарту
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptUserAccessStandardData {
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
    /// UUID пользователя
    pub(crate) user_uuid: Uuid,
    /// Идентификатор типа доступа
    pub(crate) type_access_id: i32,
}

impl From<&IptUserAccessStandardData> for InsertableUserAccessStandard {
    fn from(data_standard: &IptUserAccessStandardData) -> Self {
        let IptUserAccessStandardData {
            standard_uuid,
            user_uuid,
            type_access_id,
            ..
        } = data_standard;

        Self {
            standard_uuid: *standard_uuid,
            user_uuid: *user_uuid,
            type_access_id: *type_access_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// Данные для удаления доступа пользователя к стандарту
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct DelUserAccessStandardData {
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
    /// UUID пользователя
    pub(crate) user_uuid: Uuid,
}
