use crate::schema::*;
use crate::models::user::model::User;
use crate::models::relate_ref::file::model::DownloadFile;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Certificate for User
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(file_uuid, user_uuid)]
#[belongs_to(DownloadFile, foreign_key = "file_uuid")]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "user_certificate_ref"]
pub(crate) struct UserCertificate {
    pub(crate) file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) description: String,
}

#[derive(Debug, SimpleObject)]
pub(crate) struct UserCertificateAndFile {
    pub(crate) file: DownloadFile,
    pub(crate) user_uuid: Uuid,
    pub(crate) description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "user_certificate_ref"]
pub(crate) struct InsertableUserCertificate {
    pub(crate) file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) description: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUserCertificateData {
    pub(crate) description: String,
    pub(crate) filename: String,
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
pub(crate) struct IptUpdateUserCertificateData {
    pub(crate) file_uuid: Uuid,
    pub(crate) description: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelUserCertificateData {
    pub(crate) file_uuid: Uuid,
}
