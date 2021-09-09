use crate::errors::ServiceResult;
use crate::models::user::model::UserQuery;
use crate::models::user::certificate::model::{
    UserCertificate,
    CertificateWithSlimFile,
};
use crate::models::relate_ref::file::model::SlimFile;
use crate::schema::user_certificate_ref::dsl as user_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl SlimFile {
    /// Search files certificates target user
    pub fn for_user_certificates(
        user: &UserQuery,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SlimFile>> {
        let target_vec_uuid_file: Vec<Uuid> = UserCertificate::belonging_to(user)
            .select(user_certificate_ref::uuid_file)
            .load::<Uuid>(conn)?;

        SlimFile::get_file_by_vec_uuid(&target_vec_uuid_file, conn)
    }
}

impl CertificateWithSlimFile {
    /// Gets certificates user with slimfile data
    pub fn for_user(
        user: &UserQuery,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CertificateWithSlimFile>> {
        let certificates_user = UserCertificate::belonging_to(user)
            .load::<UserCertificate>(conn)?;

        let files_for_certificates = SlimFile::for_user_certificates(
            user,
            conn
        ).expect("Error loading certificates");

        let mut user_certificates = Vec::new();
        for cert in &certificates_user {
            for file in &files_for_certificates {
                if cert.uuid_file == file.uuid {
                    user_certificates.push(CertificateWithSlimFile{
                        file: file.to_owned(),
                        uuid_user: cert.uuid_user.to_owned(),
                        description: cert.description.to_string(),
                    })
                }
            }
        }

        Ok(user_certificates)
    }
}
