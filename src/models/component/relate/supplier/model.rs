use crate::schema::*;
use crate::models::company::model::{Company, SlimCompany};
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Supplier component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_component, uuid_company)]
#[belongs_to(Component, foreign_key = "uuid_component")]
#[belongs_to(Company, foreign_key = "uuid_company")]
#[table_name = "supplier_to_component"]
pub struct SupplierComponent {
    pub uuid_component: Uuid,
    pub uuid_company: Uuid,
    pub description: String,
}

#[Object]
impl SupplierComponent {
    async fn uuid_company(&self) -> ID {
        self.uuid_company.into()
    }
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn description(&self) -> &String {
        &self.description
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct ComponentSupplierRelatedData {
    pub supplier: SlimCompany,
    pub uuid_component: Uuid,
    pub description: String,
}

impl From<(SupplierComponent, SlimCompany)> for ComponentSupplierRelatedData {
    fn from(data: (SupplierComponent, SlimCompany)) -> Self {
        Self {
            supplier: data.1,
            uuid_component: data.0.uuid_component,
            description: data.0.description,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSupplierComponentData {
    pub uuid_component: ID,
    pub uuid_company: ID,
    pub description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "supplier_to_component"]
pub struct InsertableSupplierComponent {
    pub uuid_component: Uuid,
    pub uuid_company: Uuid,
    pub description: String,
}

impl From<IptSupplierComponentData> for InsertableSupplierComponent {
    fn from(ipt_data: IptSupplierComponentData) -> Self {
        let IptSupplierComponentData {
            uuid_component,
            uuid_company,
            description,
            ..
        } = ipt_data;

        Self {
            uuid_component: Uuid::parse_str(&uuid_component.to_string()).unwrap(),
            uuid_company: Uuid::parse_str(&uuid_company.to_string()).unwrap(),
            description,
        }
    }
}
