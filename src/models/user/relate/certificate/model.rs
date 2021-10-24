use crate::schema::*;
use crate::models::user::model::User;
use crate::models::relate_ref::file::model::ShowFileForDownload;
// use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Certificate for User
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(file_uuid, user_uuid)]
#[belongs_to(ShowFileForDownload, foreign_key = "file_uuid")]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "user_certificate_ref"]
pub struct UserCertificate {
    pub file_uuid: Uuid,
    pub user_uuid: Uuid,
    pub description: String,
}

#[derive(Debug, SimpleObject)]
pub struct CertificateAndFile {
    pub file: ShowFileForDownload,
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
    pub filename: String,
}

impl From<UserCertificate> for InsertableUserCertificate {
    fn from(ipt_data: UserCertificate) -> Self {
        let UserCertificate {
            file_uuid,
            user_uuid,
            description,
            ..
        } = ipt_data;

        Self {
            file_uuid,
            user_uuid,
            description,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUpdateUserCertificateData {
    pub file_uuid: Uuid,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelUserCertificateData {
    pub file_uuid: Uuid,
}
