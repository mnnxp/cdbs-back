use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = standard_status_ref)]
pub(crate) struct StandardStatus {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = standard_status_ref)]
pub(crate) struct InsertableStandardStatus {
    pub(crate) id: i32,
}

/// Information about the status of the standard with localization
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = standard_status_translate_list)]
pub(crate) struct StandardStatusTranslateList {
    /// Status of the standard identifier
    pub(crate) standard_status_id: i32,
    /// Language identifier
    pub(crate) lang_id: i32,
    /// Standard status name
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = standard_status_translate_list)]
pub(crate) struct InsertableStandardStatusTranslateList {
    pub(crate) standard_status_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}
