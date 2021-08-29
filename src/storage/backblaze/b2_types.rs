use async_graphql::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// URL data for uploading
pub struct UploadUrlData {
    pub bucket_id: String,
    pub upload_url: String,
    pub authorization_token: String,
}

#[Object]
impl UploadUrlData {
    async fn bucket_id(&self) -> &String {
        &self.bucket_id
    }
    async fn upload_url(&self) -> &String {
        &self.upload_url
    }
    async fn authorization_token(&self) -> &String {
        &self.authorization_token
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateKeyRequest {
    pub(crate) account_id: String,
    pub(crate) capabilities: Vec<String>,
    pub(crate) key_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) valid_duration_in_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) bucket_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) options: Option<String>,
}

impl CreateKeyRequest {
    pub(crate) fn new(
        account_id: String,
        capabilities: Vec<String>,
        key_name: String,
        valid_duration_in_seconds: Option<u64>,
        bucket_id: Option<String>,
        name_prefix: Option<String>,
        options: Option<String>,
    ) -> CreateKeyRequest {
        CreateKeyRequest {
            account_id,
            capabilities,
            key_name,
            valid_duration_in_seconds,
            bucket_id,
            name_prefix,
            options,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreatedKeyData {
    pub(crate) key_name: String,
    pub(crate) application_key_id: String,
    pub(crate) application_key: String,
    pub(crate) capabilities: Vec<String>,
    pub(crate) account_id: String,
    pub(crate) expiration_timestamp: Option<i64>,
    pub(crate) bucket_id: Option<String>,
    pub(crate) name_prefix: Option<String>,
    pub(crate) options: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AuthorizeAccountData {
    pub(crate) absolute_minimum_part_size: u32,
    pub(crate) account_id: String,
    pub(crate) allowed: AllowedToken,
    pub(crate) api_url: String,
    pub(crate) authorization_token: String,
    pub(crate) download_url: String,
    pub(crate) recommended_part_size: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AllowedToken {
    pub(crate) bucket_id: Option<String>,
    pub(crate) bucket_name: Option<String>,
    pub(crate) capabilities: Vec<String>,
    pub(crate) name_prefix: Option<String>,
}
