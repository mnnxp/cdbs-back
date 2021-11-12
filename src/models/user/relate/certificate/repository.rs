use crate::errors::ServiceResult;
use crate::models::user::certificate::model::{
    UserCertificate,
    CertificateAndFile,
};
use crate::models::relate_ref::file::model::ShowFileForDownload;
use crate::schema::user_certificate_ref::dsl as user_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFileForDownload {
    /// Search files certificates target user by user_uuid
    pub(crate) fn from_user_certificates(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFileForDownload>> {
        let target_vec_file_uuid: Vec<Uuid> = user_certificate_ref::user_certificate_ref
            .filter(user_certificate_ref::user_uuid.eq(target_user_uuid))
            .select(user_certificate_ref::file_uuid)
            .load::<Uuid>(conn)?;

        ShowFileForDownload::get_file_by_uuids(
            &target_vec_file_uuid,
            conn
        )
    }
}

impl CertificateAndFile {
    /// Gets certificates user with slimfile data by uuid
    pub(crate) fn from_user(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CertificateAndFile>> {
        let certificates_user = user_certificate_ref::user_certificate_ref
            .filter(user_certificate_ref::user_uuid.eq(target_user_uuid))
            .load::<UserCertificate>(conn)?;

        let files_for_certificates = ShowFileForDownload::from_user_certificates(
            target_user_uuid,
            conn
        ).expect("Error loading certificates");

        let mut user_certificates = Vec::new();
        for cert in &certificates_user {
            for file in &files_for_certificates {
                if cert.file_uuid == file.uuid {
                    user_certificates.push(CertificateAndFile{
                        file: file.to_owned(),
                        user_uuid: cert.user_uuid.to_owned(),
                        description: cert.description.to_string(),
                    })
                }
            }
        }

        Ok(user_certificates)
    }
}
