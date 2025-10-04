use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = service_status_ref)]
pub(crate) struct ServiceStatus {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = service_status_ref)]
pub(crate) struct InsertableServiceStatus {
    pub(crate) id: i32,
}

/// Information about the status of the service with localization
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = service_status_translate_list)]
pub(crate) struct ServiceStatusTranslateList {
    /// Status of the service identifier
    pub(crate) service_status_id: i32,
    /// Language identifier
    pub(crate) lang_id: i32,
    /// Service status name
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = service_status_translate_list)]
pub(crate) struct InsertableServiceStatusTranslateList {
    pub(crate) service_status_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}
