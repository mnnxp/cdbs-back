use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::auth::api_key::model::ApiKey;

#[derive(SimpleObject)]
pub(crate) struct ApiKeyData {
    pub(crate) id: i32,
    pub(crate) user_uuid: Uuid,
    pub(crate) name: String,
    pub(crate) last_used_at: Option<DateTime<Utc>>,
    pub(crate) expires_at: DateTime<Utc>,
    pub(crate) is_active: bool,
    pub(crate) created_at: DateTime<Utc>,
}

impl From<ApiKey> for ApiKeyData {
    fn from(key: ApiKey) -> Self {
        Self {
            id: key.id,
            user_uuid: key.user_uuid,
            name: key.name,
            last_used_at: key.last_used_at,
            expires_at: key.expires_at,
            is_active: key.is_active,
            created_at: key.created_at,
        }
    }
}
