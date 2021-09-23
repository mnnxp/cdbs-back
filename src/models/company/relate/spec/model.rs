use crate::schema::*;
use crate::models::relate_ref::spec::model::{Spec, SpecTranslateList};
use crate::models::company::model::Company;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Spec company models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(company_uuid, spec_id)]
#[belongs_to(Company, foreign_key = "company_uuid")]
#[belongs_to(Spec, foreign_key = "spec_id")]
#[table_name = "spec_to_company"]
pub struct SpecCompany {
    pub spec_id: i32,
    pub company_uuid: Uuid,
}

#[Object]
impl SpecCompany {
    async fn spec_id(&self) -> &i32 {
        &self.spec_id
    }
    async fn company_uuid(&self) -> ID {
        self.company_uuid.into()
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct CompanySpecWithTranslation {
    pub spec: SpecTranslateList,
    pub company_uuid: Uuid,
}

impl From<(SpecCompany, SpecTranslateList)> for CompanySpecWithTranslation {
    fn from(data: (SpecCompany, SpecTranslateList)) -> Self {
        Self {
            spec: data.1,
            company_uuid: data.0.company_uuid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSpecCompanyData {
    pub company_uuid: Uuid,
    pub spec_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_to_company"]
pub struct InsertableSpecCompany {
    pub company_uuid: Uuid,
    pub spec_id: i32,
}

impl From<IptSpecCompanyData> for InsertableSpecCompany {
    fn from(ipt_data: IptSpecCompanyData) -> Self {
        let IptSpecCompanyData {
            company_uuid,
            spec_id,
            ..
        } = ipt_data;

        Self {
            company_uuid: Uuid::parse_str(&company_uuid.to_string()).unwrap(),
            spec_id,
        }
    }
}
