use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::relate_ref::file::model::DownloadFile;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Certificate for Company
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[diesel(primary_key(file_uuid, company_uuid))]
#[diesel(belongs_to(DownloadFile, foreign_key = file_uuid))]
#[diesel(belongs_to(Company, foreign_key = company_uuid))]
#[diesel(table_name = company_certificate_ref)]
pub(crate) struct CompanyCertificate {
    pub(crate) file_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) description: String,
}

#[derive(Debug, SimpleObject)]
pub(crate) struct CompanyCertificateAndFile {
    pub(crate) file: DownloadFile,
    pub(crate) company_uuid: Uuid,
    pub(crate) description: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_certificate_ref)]
pub(crate) struct InsertableCompanyCertificate {
    pub(crate) file_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) description: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyCertificateData {
    pub(crate) company_uuid: Uuid,
    pub(crate) description: String,
    pub(crate) filename: String,
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
pub(crate) struct IptUpdateCompanyCertificateData {
    pub(crate) company_uuid: Uuid,
    pub(crate) file_uuid: Uuid,
    pub(crate) description: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelCompanyCertificateData {
    pub(crate) company_uuid: Uuid,
    pub(crate) file_uuid: Uuid,
}
