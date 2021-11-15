use crate::errors::ServiceResult;
use crate::models::user::certificate::model::{
    UserCertificate,
    UserCertificateAndFile,
};
use crate::models::relate_ref::file::model::DownloadFile;
use crate::schema::user_certificate_ref::dsl as user_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl DownloadFile {
    /// Search files certificates target user by user_uuid
    pub(crate) fn from_user_certificates(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<DownloadFile>> {
        let target_files_uuids: Vec<Uuid> = user_certificate_ref::user_certificate_ref
            .filter(user_certificate_ref::user_uuid.eq(target_user_uuid))
            .select(user_certificate_ref::file_uuid)
            .load::<Uuid>(conn)?;

        DownloadFile::get_by_files_uuids(
            &target_files_uuids,
            conn
        )
    }
}

impl UserCertificateAndFile {
    /// Gets certificates user with slimfile data by uuid
    pub(crate) fn from_user(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<UserCertificateAndFile>> {
        let certificates_user = user_certificate_ref::user_certificate_ref
            .filter(user_certificate_ref::user_uuid.eq(target_user_uuid))
            .load::<UserCertificate>(conn)?;

        let files_for_certificates = DownloadFile::from_user_certificates(
            target_user_uuid,
            conn
        ).expect("Error loading certificates");

        let mut user_certificates = Vec::new();
        for cert in &certificates_user {
            for file in &files_for_certificates {
                if cert.file_uuid == file.uuid {
                    user_certificates.push(UserCertificateAndFile{
                        file: file.to_owned(),
                        user_uuid: cert.user_uuid.to_owned(),
                        description: cert.description.to_string(),
                    });
                    break;
                }
            }
        }

        Ok(user_certificates)
    }
}
