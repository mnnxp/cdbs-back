use crate::schema::*;
use chrono::*;
// use uuid::Uuid;

#[derive(Insertable, Serialize, Deserialize, Queryable, Clone, Debug)]
#[table_name = "storage_access_ref"]
pub(crate) struct StorageAccess {
    id: i32,
    application_key_id: String,
    application_key: String,
    expiration_at: NaiveDateTime,
    pub(crate) bucket: String,
    pub(crate) region: String,
    pub(crate) endpoint: String,
}

impl From<&StorageAccess> for super::s3::Aws {
    fn from(data: &StorageAccess) -> super::s3::Aws {
        // todo!(add encryptind)
        let application_key_id = &data.application_key_id;
        let application_key = &data.application_key;

        super::s3::Aws::new(
            application_key_id,
            application_key,
            &data.region,
            &data.endpoint
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct FileHeaders {
    pub(crate) content_type: Option<String>,
    pub(crate) content_length: Option<i64>,
    pub(crate) updated_at: Option<NaiveDateTime>,
}

impl From<rusoto_s3::HeadObjectOutput> for FileHeaders {
    fn from(data: rusoto_s3::HeadObjectOutput) -> Self {
        let rusoto_s3::HeadObjectOutput{
            content_length,
            content_type,
            last_modified,
            ..
        } = data;

        Self{
            content_length,
            content_type,
            updated_at: last_modified.map(|date_str| NaiveDateTime::parse_from_str(date_str.as_str(), "%a, %d %b %Y %H:%M:%S GMT").unwrap()),
        }
    }
}
