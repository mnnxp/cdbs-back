use crate::schema::*;
use crate::models::user::model::UserQuery;
use crate::models::relate_ref::file::model::{ShowFile, SlimFile};
use async_graphql::*;
use uuid::Uuid;

// Certificate for User
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(file_uuid, user_uuid)]
#[belongs_to(ShowFile, foreign_key = "file_uuid")]
#[belongs_to(UserQuery, foreign_key = "user_uuid")]
#[table_name = "user_certificate_ref"]
pub struct UserCertificate {
    pub file_uuid: Uuid,
    pub user_uuid: Uuid,
    pub description: String,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct CertificateWithSlimFile {
    pub file: SlimFile,
    pub user_uuid: Uuid,
    pub description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "user_certificate_ref"]
pub struct InsertableUserCertificate {
    pub file_uuid: Uuid,
    pub user_uuid: Uuid,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUserCertificateData {
    pub description: String,
}
