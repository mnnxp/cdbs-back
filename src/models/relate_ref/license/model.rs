use crate::schema::*;
use async_graphql::*;
use chrono::*;

/// Данные о лицензии распространения
#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = license_ref)]
pub(crate) struct License {
    /// Идентификатор лицензии
    pub(crate) id: i32,
    /// Наименование лицензии
    pub(crate) name: String,
    /// Аббревиатура или сокращение лицензии
    pub(crate) keyword: String,
    /// Дата публикации основного текста лицензии
    pub(crate) publication_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = license_ref)]
pub(crate) struct InsertableLicense {
    pub(crate) name: String,
    pub(crate) keyword: String,
    pub(crate) publication_at: NaiveDateTime,
}

/// Данные для запроса на добавлении лицензии распространения в базу данных
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct LicenseData {
    /// Наименование лицензии
    pub(crate) name: String,
    /// Аббревиатура или сокращение лицензии
    pub(crate) keyword: String,
    /// Дата публикации основного текста лицензии
    pub(crate) publication_at: NaiveDateTime,
}

/// Сокращённые данные о лицензии распространения
#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub(crate) struct SlimLicense {
    /// Идентификатор лицензии
    pub(crate) id: i32,
    /// Аббревиатура или сокращение лицензии
    pub(crate) keyword: String,
}

impl From<&LicenseData> for InsertableLicense {
    fn from(data: &LicenseData) -> Self {
        Self {
            name: data.name.clone(),
            keyword: data.keyword.clone(),
            publication_at: data.publication_at,
        }
    }
}

/// Аргументы для запроса существующий на платформе лицензий распространения
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptLicenseArg {
    /// Фильтр о идентификаторам лицензий
    pub(crate) license_ids:  Option<Vec<i32>>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct LicenseArg {
    pub(crate) license_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for LicenseArg {
    fn default() -> Self {
        Self {
            license_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptLicenseArg> for LicenseArg {
    fn from(data: IptLicenseArg) -> Self {
        let IptLicenseArg {
            license_ids,
            limit,
            offset,
        } = data;

        Self {
            license_ids: license_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
