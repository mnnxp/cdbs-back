use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::relate_ref::file::model::ShowFile;
// use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Certificate for Company
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(file_uuid, company_uuid)]
#[belongs_to(ShowFile, foreign_key = "file_uuid")]
#[belongs_to(Company, foreign_key = "company_uuid")]
#[table_name = "company_certificate_ref"]
pub struct CompanyCertificate {
    pub file_uuid: Uuid,
    pub company_uuid: Uuid,
    pub description: String,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct CertificateWithShowFile {
    pub file: ShowFile,
    pub company_uuid: Uuid,
    pub description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "company_certificate_ref"]
pub struct InsertableCompanyCertificate {
    pub file_uuid: Uuid,
    pub company_uuid: Uuid,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptCompanyCertificateData {
    pub company_uuid: Uuid,
    pub description: String,
    pub filename: String,
}

impl From<CompanyCertificate> for InsertableCompanyCertificate {
    fn from(ipt_data: CompanyCertificate) -> Self {
        let CompanyCertificate {
            file_uuid,
            company_uuid,
            description,
            ..
        } = ipt_data;

        Self {
            file_uuid,
            company_uuid,
            description,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUpdateCompanyCertificateData {
    pub company_uuid: Uuid,
    pub file_uuid: Uuid,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelCompanyCertificateData {
    pub company_uuid: Uuid,
    pub file_uuid: Uuid,
}
