use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = actual_status_ref)]
pub(crate) struct ActualStatus {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = actual_status_ref)]
pub(crate) struct InsertableActualStatus {
    pub(crate) id: i32,
}

/// Actual status is information about the stage of the component's (product's) life cycle
/// with localization (translation) for the specified language
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = actual_status_translate_list)]
pub(crate) struct ActualStatusTranslateList {
    /// Actual status identifier
    pub(crate) actual_status_id: i32,
    /// Localization language identifier
    pub(crate) lang_id: i32,
    /// Name of the current status
    pub(crate) name: String,
}