use crate::errors::ServiceResult;
use crate::models::company::certificate::model::{
    CompanyCertificate,
    CompanyCertificateAndFile,
};
use crate::models::relate_ref::file::model::DownloadFile;
use crate::schema::company_certificate_ref::dsl as company_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl DownloadFile {
    /// Search files certificates target company by uuid
    pub(crate) fn from_company_certificates(
        target_company_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<DownloadFile>> {
        let target_files_uuids: Vec<Uuid> = company_certificate_ref::company_certificate_ref
            .filter(company_certificate_ref::company_uuid.eq(target_company_uuid))
            .select(company_certificate_ref::file_uuid)
            .load::<Uuid>(conn)?;

        DownloadFile::get_by_files_uuids(
            &target_files_uuids,
            conn
        )
    }
}

impl CompanyCertificateAndFile {
    /// Gets certificates company with slimfile data
    pub(crate) fn from_company(
        target_company_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyCertificateAndFile>> {
        let certificates_company = company_certificate_ref::company_certificate_ref
            .filter(company_certificate_ref::company_uuid.eq(target_company_uuid))
            .load::<CompanyCertificate>(conn)?;

        let files_for_certificates = DownloadFile::from_company_certificates(
            target_company_uuid,
            conn
        ).expect("Error loading certificates");

        let mut company_certificates = Vec::new();
        for cert in &certificates_company {
            for file in &files_for_certificates {
                if cert.file_uuid == file.uuid {
                    company_certificates.push(CompanyCertificateAndFile{
                        file: file.to_owned(),
                        company_uuid: cert.company_uuid.to_owned(),
                        description: cert.description.to_string(),
                    })
                }
            }
        }

        Ok(company_certificates)
    }
}
