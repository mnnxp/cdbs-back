use crate::schema::*;
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
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
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
