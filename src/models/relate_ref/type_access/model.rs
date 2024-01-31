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

/// Access type information with localization
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[diesel(primary_key(type_access_id, lang_id))]
#[diesel(belongs_to(TypeAccess, foreign_key = type_access_id))]
#[diesel(belongs_to(Component, foreign_key = type_access_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = type_access_translate_list)]
pub(crate) struct TypeAccessTranslateList {
    /// Access type identifier
    pub(crate) type_access_id: i32,
    /// Name localization language identifier
    pub(crate) lang_id: i32,
    /// Localized name of the access type
    pub(crate) name: String,
}

/// Data for request to add access type
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptTypeAccessTranslateListData {
    /// Name localization language identifier
    pub(crate) lang_id: i32,
    /// Localized name of the access type
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = type_access_translate_list)]
pub(crate) struct InsertableTypeAccessTranslateList {
    pub(crate) type_access_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

/// Arguments for requesting available access types
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptTypeAccessArg {
    /// Filtering by access type identifiers
    pub(crate) type_access_ids: Option<Vec<i32>>,
    /// Restriction of data sampling (maximum number of records)
    pub(crate) limit: Option<i32>,
    /// Number of skipping records at the beginning (offset)
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
