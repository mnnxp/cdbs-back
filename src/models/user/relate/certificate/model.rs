use crate::schema::*;
use crate::models::relate_ref::file::model::DownloadFile;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Certificate for User
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = user_certificate_ref)]
pub(crate) struct UserCertificate {
    pub(crate) file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) description: String,
}

/// User certificate data
#[derive(Debug, SimpleObject)]
pub(crate) struct UserCertificateAndFile {
    /// Data for obtaining a certificate file
    pub(crate) file: DownloadFile,
    /// User's UUID
    pub(crate) user_uuid: Uuid,
    /// User certificate description
    pub(crate) description: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_certificate_ref)]
pub(crate) struct InsertableUserCertificate {
    pub(crate) file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) description: String,
}

/// Data for requesting to add a user certificate
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUserCertificateData {
    /// User certificate description
    pub(crate) description: String,
    /// Name of the certificate file
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

/// Data for user certificate renewal request
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateUserCertificateData {
    /// UUID of the certificate
    pub(crate) file_uuid: Uuid,
    /// New certificate description
    pub(crate) description: String,
}

/// Data for user certificate deletion request
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelUserCertificateData {
    /// UUID of the certificate file to be deleted
    pub(crate) file_uuid: Uuid,
}
