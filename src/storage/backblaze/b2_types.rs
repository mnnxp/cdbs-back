use async_graphql::*;
use chrono::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct UploadUrl  {
    #[serde(rename = "Authorization")]
    pub authorization: String,
    #[serde(rename = "X-Bz-File-Name")]
    pub file_name: String,
    #[serde(rename = "Content-Type")]
    pub content_type: String,
    #[serde(rename = "X-Bz-Content-Sha1")]
    pub content_sha1: String,
    #[serde(rename = "X-Bz-Server-Side-Encryption")]
    pub server_side_encryption: String,
    #[serde(rename = "Upload-URL")]
    pub upload_url: String,
}

// -H "Authorization: $UPLOAD_AUTHORIZATION_TOKEN" \
// -H "X-Bz-File-Name: $FILE_TO_UPLOAD" \
// -H "Content-Type: $MIME_TYPE" \
// -H "X-Bz-Content-Sha1: $SHA1_OF_FILE" \
// -H "X-Bz-Info-Author: unknown" \
// -H "X-Bz-Server-Side-Encryption: AES256" \

// curl \
// -H "Authorization": "4_002cb0e1d5d32050000000012_019eb445_c58b88_upld_Lkqg_k0sdKRNPGbThnDVncW3ZVg=" \
// -H "X-Bz-File-Name": "4f5eb24dfc9901db49c929e6aff5164afc7dc3ef" \
// -H "Content-Type": "b2/x-auto" \
// -H "X-Bz-Content-Sha1": "0c7c609f-0995-4fc6-adb2-a5a976486e15/0c7c609f-0995-4fc6-adb2-a5a976486e15-a4e678e7-f71d-46c3-a6ba-478efc60b6a0" \
// -H "X-Bz-Server-Side-Encryption: AES256" \
// --data-binary "@typing_test.dwg" \
// "https://pod-000-1162-00.backblaze.com/b2api/v2/b2_upload_file/1c8ba08e816d056d73b20015/c002_v0001162_t0035"

#[Object]
impl UploadUrl {
    async fn authorization(&self) -> &String {
        &self.authorization
    }
    async fn file_name(&self) -> &String {
        &self.file_name
    }
    async fn content_type(&self) -> &String {
        &self.content_type
    }
    async fn content_sha1(&self) -> &String {
        &self.content_sha1
    }
    async fn server_side_encryption(&self) -> &String {
        &self.server_side_encryption
    }
    async fn upload_url(&self) -> &String {
        &self.upload_url
    }
}

/// URL data for uploading
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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

/// For ets only the headers information of file.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileHeaders {
    pub cache_control: String, // cache-control "max-age=0, no-cache, no-store",
    pub file_name: String, // x-bz-file-name "0c7c609f09954fc6adb2a5a976486e15/38e478ee191d49dabafeef5453bfeb63",
    pub file_id: String, // x-bz-file-id "4_z1c8ba08e816d056d73b20015_f1119b4a9c64b7621_d20210902_m162659_c002_v0001152_t0007",
    pub content_sha1: String, // x-bz-content-sha1 "4f5eb24dfc9901db49c929e6aff5164afc7dc3ef",
    pub upload_timestamp: String, // x-bz-upload-timestamp "1630600019000",
    pub accept_ranges: String, // accept-ranges "bytes",
    pub server_side_encryption: String, // x-bz-server-side-encryption "AES256",
    pub content_type: String, // content-type "application/octet-stream",
    pub content_length: String, // content-length "33",
    pub date: NaiveDateTime, // date "Thu, 02 Sep 2021 19:26:19 GMT",
}

impl Default for FileHeaders {
    fn default() -> Self {
        let date = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
        Self {
            cache_control: String::new(),
            file_name: String::new(),
            file_id: String::new(),
            content_sha1: String::new(),
            upload_timestamp: String::new(),
            accept_ranges: String::new(),
            server_side_encryption: String::new(),
            content_type: String::new(),
            content_length: String::new(),
            date,
        }
    }
}

/// Getting file metadata in B2 storage from headers
impl From<&reqwest::Response> for FileHeaders {
    fn from(response: &reqwest::Response) -> Self {
        let mut headers_file: FileHeaders = FileHeaders::default();
        for (key, value) in response.headers().iter() {
            // debug!("{:?}: {:?}", key, value);
            match key.as_str() {
                "cache-control" => headers_file.cache_control = value.to_str().unwrap().to_string(),
                "x-bz-file-name" => headers_file.file_name = value.to_str().unwrap().to_string(),
                "x-bz-file-id" => headers_file.file_id = value.to_str().unwrap().to_string(),
                "x-bz-content-sha1" => headers_file.content_sha1 = value.to_str().unwrap().to_string(),
                "x-bz-upload-timestamp" => headers_file.upload_timestamp = value.to_str().unwrap().to_string(),
                "accept-ranges" => headers_file.accept_ranges = value.to_str().unwrap().to_string(),
                "x-bz-server-side-encryption" => headers_file.server_side_encryption = value.to_str().unwrap().to_string(),
                "content-type" => headers_file.content_type = value.to_str().unwrap().to_string(),
                "content-length" => headers_file.content_length = value.to_str().unwrap().to_string(),
                "date" => headers_file.date = NaiveDateTime::parse_from_str(value.to_str().unwrap(), "%a, %d %b %Y %H:%M:%S GMT").unwrap(),
                _ => debug!("Not match: {:?}", key),
            }
        }

        headers_file
    }
}

/// For response after file successfully uploaded.
/// The response will contain the standard file information.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseForUploadFile {
    pub file_id: String,
    pub file_name: String,
    pub account_id: String,
    pub bucket_id: String,
    // pub content_length: u64,
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
    // async fn content_length(&self) -> u64 {
    //     self.content_length
    // }
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

/// substruct for ResponseForUploadFile
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub author: String,
}

#[Object]
impl Author {
    async fn author(&self) -> &String {
        &self.author
    }
}

/// substruct for ResponseForUploadFile
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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

/// substruct for ResponseForUploadFile
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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

/// substruct for ResponseForUploadFile
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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

/// substruct for ResponseForUploadFile
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
