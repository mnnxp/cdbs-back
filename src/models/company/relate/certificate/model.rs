use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::relate_ref::file::model::{ShowFile, SlimFile};
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Certificate for Company
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(uuid_file, uuid_company)]
#[belongs_to(ShowFile, foreign_key = "uuid_file")]
#[belongs_to(Company, foreign_key = "uuid_company")]
#[table_name = "company_certificate_ref"]
pub struct CompanyCertificate {
    pub uuid_file: Uuid,
    pub uuid_company: Uuid,
    pub description: String,
}

#[Object]
impl CompanyCertificate {
    async fn uuid_file(&self) -> ID {
        self.uuid_file.into()
    }
    async fn uuid_company(&self) -> ID {
        self.uuid_company.into()
    }
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct CertificateWithSlimFile {
    pub file: SlimFile,
    pub uuid_company: Uuid,
    pub description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "company_certificate_ref"]
pub struct InsertableCompanyCertificate {
    pub uuid_file: Uuid,
    pub uuid_company: Uuid,
    pub description: String,
}

impl From<CompanyCertificate> for InsertableCompanyCertificate {
    fn from(ipt_data: CompanyCertificate) -> Self {
        let CompanyCertificate {
            uuid_file,
            uuid_company,
            description,
            ..
        } = ipt_data;

        Self {
            uuid_file,
            uuid_company,
            description,
        }
    }
}
