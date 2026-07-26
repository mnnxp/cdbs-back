use crate::schema::user_api_key_ref;
use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = user_api_key_ref)]
pub(crate) struct ApiKey {
    pub(crate) id: i32,
    pub(crate) user_uuid: Uuid,
    pub(crate) key_hash: Vec<u8>,
    pub(crate) name: String,
    pub(crate) last_used_at: Option<DateTime<Utc>>,
    pub(crate) expires_at: DateTime<Utc>,
    pub(crate) is_active: bool,
    pub(crate) created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = user_api_key_ref)]
pub(crate) struct NewApiKey {
    pub(crate) user_uuid: Uuid,
    pub(crate) key_hash: Vec<u8>,
    pub(crate) name: String,
    pub(crate) expires_at: DateTime<Utc>,
}

/// Data for updating an API key
#[derive(Debug, InputObject, AsChangeset)]
#[diesel(table_name = user_api_key_ref)]
pub(crate) struct IptUpdateApiKeyData {
    /// New name for the API key
    pub(crate) name: Option<String>,
    /// New expiration date (None = 1 year expires)
    pub(crate) expires_at: Option<DateTime<Utc>>,
    /// Whether the key is active
    pub(crate) is_active: Option<bool>,
}
