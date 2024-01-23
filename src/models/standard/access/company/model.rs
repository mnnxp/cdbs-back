use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::{InputObject, SimpleObject};
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(standard_uuid, company_uuid))]
#[diesel(table_name = company_access_to_standard)]
pub(crate) struct CompanyAccessStandard {
    pub(crate) standard_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Данные о наличии доступа к стандарту у компании (участников компании)
#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct CompanyAccessStandardAndRelatedData {
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
    /// UUID имеющей доступ компании
    pub(crate) company_uuid: Uuid,
    /// Тип доступа с локализацией
    pub(crate) type_access: TypeAccessTranslateList,
    /// Флаг активности доступа
    pub(crate) is_enabled: bool,
    /// Дата первой выдачи доступа компании
    pub(crate) created_at: NaiveDateTime,
    /// Дата обновления доступа
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_access_to_standard)]
pub(crate) struct InsertableCompanyAccessStandard {
    pub(crate) standard_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Данные для запроса на выдачу (участникам) компании доступа к стандарту
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptCompanyAccessStandardData {
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
    /// UUID компании
    pub(crate) company_uuid: Uuid,
    /// Идентификатор типа доступа
    pub(crate) type_access_id: i32,
}

impl From<&IptCompanyAccessStandardData> for InsertableCompanyAccessStandard {
    fn from(data_standard: &IptCompanyAccessStandardData) -> Self {
        let IptCompanyAccessStandardData {
            standard_uuid,
            company_uuid,
            type_access_id,
            ..
        } = data_standard;

        Self {
            standard_uuid: *standard_uuid,
            company_uuid: *company_uuid,
            type_access_id: *type_access_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// Данные для удаления доступа компании к стандарту
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct DelCompanyAccessStandardData {
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
    /// UUID компании
    pub(crate) company_uuid: Uuid,
}
