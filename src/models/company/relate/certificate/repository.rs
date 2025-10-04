use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::certificate::model::{CompanyCertificate, CompanyCertificateAndFile};
use crate::models::relate_ref::file::model::DownloadFile;
use crate::schema::company_certificate_ref::dsl as company_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl CompanyCertificateAndFile {
    /// Gets certificates company with slimfile data
    pub(crate) fn from_company(
        target_company_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<CompanyCertificateAndFile>> {
        let certificates_company = company_certificate_ref::company_certificate_ref
            .filter(company_certificate_ref::company_uuid.eq(target_company_uuid))
            .load::<CompanyCertificate>(conn)
            .map_err(|err| {
                debug!("Fail get files for company_certificate_ref: {:?} ", err);
                ServiceError::InternalServerError
            })?;

        let mut company_certificates = Vec::new();
        for cert in &certificates_company {
            let file = DownloadFile::get_by_file_uuid(&cert.file_uuid, conn)?;
            company_certificates.push(CompanyCertificateAndFile {
                file: file.clone(),
                company_uuid: cert.company_uuid,
                description: cert.description.clone(),
            })
        }

        Ok(company_certificates)
    }
}
