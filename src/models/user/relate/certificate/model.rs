use crate::schema::*;
use crate::models::user::model::User;
use crate::models::relate_ref::file::model::DownloadFile;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Certificate for User
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[diesel(primary_key(file_uuid, user_uuid))]
#[diesel(belongs_to(DownloadFile, foreign_key = file_uuid))]
#[diesel(belongs_to(User, foreign_key = user_uuid))]
#[diesel(table_name = user_certificate_ref)]
pub(crate) struct UserCertificate {
    pub(crate) file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) description: String,
}

/// Данные о сертификате пользователя
#[derive(Debug, SimpleObject)]
pub(crate) struct UserCertificateAndFile {
    /// Данные для получения файла сертификата
    pub(crate) file: DownloadFile,
    /// UUID пользователя
    pub(crate) user_uuid: Uuid,
    /// Описание сертификата пользователя
    pub(crate) description: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_certificate_ref)]
pub(crate) struct InsertableUserCertificate {
    pub(crate) file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) description: String,
}

/// Данные для запроса на добавление сертификата пользователя
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUserCertificateData {
    /// Описание сертификата пользователя
    pub(crate) description: String,
    /// Наименование файла с сертификатом
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

/// Данные для запроса на обновление опсания сертификата пользователя
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateUserCertificateData {
    /// UUID файла сертификата
    pub(crate) file_uuid: Uuid,
    /// Новое описание сертификата
    pub(crate) description: String,
}

/// Данные для запроса на удаление сертификата пользователя
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelUserCertificateData {
    /// UUID файла сертификата для удаления
    pub(crate) file_uuid: Uuid,
}
