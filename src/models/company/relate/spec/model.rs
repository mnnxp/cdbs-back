use crate::schema::*;
use crate::models::relate_ref::spec::model::{Spec, SpecTranslateList};
use crate::models::company::model::Company;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Spec company models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_company, id_spec)]
#[belongs_to(Company, foreign_key = "uuid_company")]
#[belongs_to(Spec, foreign_key = "id_spec")]
#[table_name = "spec_to_company"]
pub struct SpecCompany {
    pub id_spec: i32,
    pub uuid_company: Uuid,
}

#[Object]
impl SpecCompany {
    async fn id_spec(&self) -> &i32 {
        &self.id_spec
    }
    async fn uuid_company(&self) -> ID {
        self.uuid_company.into()
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct CompanySpecWithTranslation {
    pub spec: SpecTranslateList,
    pub uuid_company: Uuid,
}

impl From<(SpecCompany, SpecTranslateList)> for CompanySpecWithTranslation {
    fn from(data: (SpecCompany, SpecTranslateList)) -> Self {
        Self {
            spec: data.1,
            uuid_company: data.0.uuid_company,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSpecCompanyData {
    pub uuid_company: ID,
    pub id_spec: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_to_company"]
pub struct InsertableSpecCompany {
    pub uuid_company: Uuid,
    pub id_spec: i32,
}

impl From<IptSpecCompanyData> for InsertableSpecCompany {
    fn from(ipt_data: IptSpecCompanyData) -> Self {
        let IptSpecCompanyData {
            uuid_company,
            id_spec,
            ..
        } = ipt_data;

        Self {
            uuid_company: Uuid::parse_str(&uuid_company.to_string()).unwrap(),
            id_spec,
        }
    }
}
