use crate::errors::ServiceResult;
use crate::models::company::model::Company;
use crate::models::company::certificate::model::{
    CompanyCertificate,
    CertificateWithShowFile,
};
use crate::models::relate_ref::file::model::ShowFile;
use crate::schema::company_certificate_ref::dsl as company_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFile {
    /// Search files certificates target company
    pub fn for_company_certificates(
        company: &Company,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFile>> {
        let target_vec_file_uuid: Vec<Uuid> = CompanyCertificate::belonging_to(company)
            .select(company_certificate_ref::file_uuid)
            .load::<Uuid>(conn)?;

        ShowFile::get_file_by_vec_uuid(&target_vec_file_uuid, conn)
    }
}

impl CertificateWithShowFile {
    /// Gets certificates company with slimfile data
    pub fn for_company(
        company: &Company,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CertificateWithShowFile>> {
        let certificates_company = CompanyCertificate::belonging_to(company)
            .load::<CompanyCertificate>(conn)?;

        let files_for_certificates = ShowFile::for_company_certificates(
            company,
            conn
        ).expect("Error loading certificates");

        let mut company_certificates = Vec::new();
        for cert in &certificates_company {
            for file in &files_for_certificates {
                if cert.file_uuid == file.uuid {
                    company_certificates.push(CertificateWithShowFile{
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
