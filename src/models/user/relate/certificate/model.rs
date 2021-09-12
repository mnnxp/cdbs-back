use crate::schema::*;
use crate::models::user::model::UserQuery;
use crate::models::relate_ref::file::model::{ShowFile, SlimFile};
use async_graphql::*;
use uuid::Uuid;

// Certificate for User
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(uuid_file, uuid_user)]
#[belongs_to(ShowFile, foreign_key = "uuid_file")]
#[belongs_to(UserQuery, foreign_key = "uuid_user")]
#[table_name = "user_certificate_ref"]
pub struct UserCertificate {
    pub uuid_file: Uuid,
    pub uuid_user: Uuid,
    pub description: String,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct CertificateWithSlimFile {
    pub file: SlimFile,
    pub uuid_user: Uuid,
    pub description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "user_certificate_ref"]
pub struct InsertableUserCertificate {
    pub uuid_file: Uuid,
    pub uuid_user: Uuid,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUserCertificateData {
    pub description: String,
}
