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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// For response after file successfully uploaded.
/// The response will contain the standard file information.
pub struct ResponseForUploadFile {
    pub file_id: String,
    pub file_name: String,
    pub account_id: String,
    pub bucket_id: String,
    pub content_length: i64,
    pub content_sha1: String,
    pub content_type: String,
    pub file_info: Author,
    pub file_retention: FileRetention,
    pub legal_hold: LegalHold,
    pub server_side_encryption: ServerSideEncryption,
}

#[Object]
impl ResponseForUploadFile {
    async fn file_id(&self) -> &String {
        &self.file_id
    }
    async fn file_name(&self) -> &String {
        &self.file_name
    }
    async fn account_id(&self) -> &String {
        &self.account_id
    }
    async fn bucket_id(&self) -> &String {
        &self.bucket_id
    }
    async fn content_length(&self) -> i64 {
        self.content_length
    }
    async fn content_sha1(&self) -> &String {
        &self.content_sha1
    }
    async fn content_type(&self) -> &String {
        &self.content_type
    }
    async fn file_info(&self) -> &Author {
        &self.file_info
    }
    async fn file_retention(&self) -> &FileRetention {
        &self.file_retention
    }
    async fn legal_hold(&self) -> &LegalHold {
        &self.legal_hold
    }
    async fn server_side_encryption(&self) -> &ServerSideEncryption {
        &self.server_side_encryption
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// substruct for ResponseForUploadFile
pub struct Author {
    pub author: String,
}

#[Object]
impl Author {
    async fn author(&self) -> &String {
        &self.author
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// substruct for ResponseForUploadFile
pub struct FileRetention {
    pub is_client_authorized_to_read: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<FileRetentionValue>,
}

#[Object]
impl FileRetention {
    async fn is_client_authorized_to_read(&self) -> bool {
        self.is_client_authorized_to_read
    }
    async fn value(&self) -> &Option<FileRetentionValue> {
        &self.value
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// substruct for ResponseForUploadFile
pub struct FileRetentionValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_until_timestamp: Option<i64>,
}

#[Object]
impl FileRetentionValue {
    async fn mode(&self) -> &Option<String> {
        &self.mode
    }
    async fn retain_until_timestamp(&self) -> &Option<i64> {
        &self.retain_until_timestamp
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// substruct for ResponseForUploadFile
pub struct LegalHold {
    pub is_client_authorized_to_read: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[Object]
impl LegalHold {
    async fn is_client_authorized_to_read(&self) -> bool {
        self.is_client_authorized_to_read
    }
    async fn value(&self) -> &Option<String> {
        &self.value
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// substruct for ResponseForUploadFile
pub struct ServerSideEncryption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[Object]
impl ServerSideEncryption {
    async fn algorithm(&self) -> &Option<String> {
        &self.algorithm
    }
    async fn mode(&self) -> &Option<String> {
        &self.mode
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
