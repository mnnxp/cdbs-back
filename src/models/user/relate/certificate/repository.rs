use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::certificate::model::{
    UserCertificate, UserCertificateAndFile,
};
use crate::models::relate_ref::file::model::DownloadFile;
use crate::schema::user_certificate_ref::dsl as user_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl UserCertificateAndFile {
    /// Gets certificates user with slimfile data by uuid
    pub(crate) fn from_user(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<UserCertificateAndFile>> {
        let certificates_user = user_certificate_ref::user_certificate_ref
            .filter(user_certificate_ref::user_uuid.eq(target_user_uuid))
            .load::<UserCertificate>(conn)
            .map_err(|err| {
                debug!("Fail get files for user_certificate_ref: {:?} ", err);
                ServiceError::InternalServerError
            })?;

        let mut user_certificates = Vec::new();
        for cert in &certificates_user {
            let file = DownloadFile::get_by_file_uuid(&cert.file_uuid, conn)?;
            user_certificates.push(UserCertificateAndFile{
                file: file.clone(),
                user_uuid: cert.user_uuid,
                description: cert.description.clone(),
            });
        }

        Ok(user_certificates)
    }
}
