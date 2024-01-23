use crate::schema::*;
// use crate::models::company::model::Company;
use crate::models::component::model::Component;
// use crate::models::standard::model::Standard;
// use crate::models::user::model::UserQuery;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

// TypeAccess models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = type_access_ref)]
pub(crate) struct TypeAccess {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = type_access_ref)]
pub(crate) struct InsertableTypeAccess {
    pub(crate) id: i32,
}

/// Информация о типе доступа с локализацией
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[diesel(primary_key(type_access_id, lang_id))]
#[diesel(belongs_to(TypeAccess, foreign_key = type_access_id))]
#[diesel(belongs_to(Component, foreign_key = type_access_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = type_access_translate_list)]
pub(crate) struct TypeAccessTranslateList {
    /// Идентификатор типа доступа
    pub(crate) type_access_id: i32,
    /// Идентификатор языка локализации наименования
    pub(crate) lang_id: i32,
    /// Локализованное наименование типа доступа
    pub(crate) name: String,
}

/// Данные для запроса на добавление типа доступа
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptTypeAccessTranslateListData {
    /// Идентификатор языка локализации наименования
    pub(crate) lang_id: i32,
    /// Локализованное наименование типа доступа
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = type_access_translate_list)]
pub(crate) struct InsertableTypeAccessTranslateList {
    pub(crate) type_access_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

/// Аргументы для запроса доступных типов доступа
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptTypeAccessArg {
    /// Фильтрация по идентификаторам типа доступа
    pub(crate) type_access_ids:  Option<Vec<i32>>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct TypeAccessArg {
    pub(crate) type_access_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for TypeAccessArg {
    fn default() -> Self {
        Self {
            type_access_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptTypeAccessArg> for TypeAccessArg {
    fn from(data: IptTypeAccessArg) -> Self {
        let IptTypeAccessArg {
            type_access_ids,
            limit,
            offset,
        } = data;

        Self {
            type_access_ids: type_access_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
