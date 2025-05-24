use crate::schema::*;
use crate::models::relate_ref::file::model::DownloadFile;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Certificate for Company
#[derive(Serialize, Deserialize, Queryable, Debug)]
#[diesel(table_name = company_certificate_ref)]
pub(crate) struct CompanyCertificate {
    pub(crate) file_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) description: String,
}

/// Company certificate file data and description
#[derive(Debug, SimpleObject)]
pub(crate) struct CompanyCertificateAndFile {
    /// Data for displaying the certificate file
    pub(crate) file: DownloadFile,
    /// UUID of the company to which the certificate belongs
    pub(crate) company_uuid: Uuid,
    /// Certificate description
    pub(crate) description: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_certificate_ref)]
pub(crate) struct InsertableCompanyCertificate {
    pub(crate) file_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) description: String,
}

/// Data for adding a new certificate
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyCertificateData {
    /// UUID of the company to which the certificate belongs
    pub(crate) company_uuid: Uuid,
    /// Certificate description
    pub(crate) description: String,
    /// Name of the uploaded file
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

/// Data for updating the company certificate description
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateCompanyCertificateData {
    /// UUID of the company to which the certificate belongs
    pub(crate) company_uuid: Uuid,
    /// File ID of the certificate to be updated
    pub(crate) file_uuid: Uuid,
    /// New description for the certificate
    pub(crate) description: String,
}

/// Data for deleting a company certificate
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelCompanyCertificateData {
    /// UUID of the company to which the certificate belongs
    pub(crate) company_uuid: Uuid,
    /// File ID of the certificate to be renewed
    pub(crate) file_uuid: Uuid,
}
