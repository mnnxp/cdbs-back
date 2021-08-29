use crate::schema::*;
use chrono::*;
use uuid::Uuid;

#[derive(Insertable, Serialize, Deserialize, Queryable, Clone, Debug)]
#[table_name = "user_storage_access_ref"]
pub(crate) struct UserStorageAccess {
    pub(crate) uuid_user: Uuid,
    pub(crate) application_key_id: String,
    pub(crate) application_key: String,
    pub(crate) key_expiration_at: NaiveDateTime,
    pub(crate) bucket_id: String,
    pub(crate) api_url: String,
    pub(crate) authorization_token: String,
    pub(crate) token_expiration_at: NaiveDateTime,
}
