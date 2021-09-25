use crate::schema::*;
use crate::models::company::model::{Company, SlimCompany};
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Supplier component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(component_uuid, company_uuid)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(Company, foreign_key = "company_uuid")]
#[table_name = "supplier_to_component"]
pub struct SupplierComponent {
    pub component_uuid: Uuid,
    pub company_uuid: Uuid,
    pub description: String,
}

#[Object]
impl SupplierComponent {
    async fn company_uuid(&self) -> ID {
        self.company_uuid.into()
    }
    async fn component_uuid(&self) -> ID {
        self.component_uuid.into()
    }
    async fn description(&self) -> &String {
        &self.description
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct ComponentSupplierRelatedData {
    pub supplier: SlimCompany,
    pub component_uuid: Uuid,
    pub description: String,
}

impl From<(SupplierComponent, SlimCompany)> for ComponentSupplierRelatedData {
    fn from(data: (SupplierComponent, SlimCompany)) -> Self {
        Self {
            supplier: data.1,
            component_uuid: data.0.component_uuid,
            description: data.0.description,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSupplierComponentData {
    pub component_uuid: Uuid,
    pub company_uuid: Uuid,
    pub description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "supplier_to_component"]
pub struct InsertableSupplierComponent {
    pub component_uuid: Uuid,
    pub company_uuid: Uuid,
    pub description: String,
}

impl From<&IptSupplierComponentData> for InsertableSupplierComponent {
    fn from(ipt_data: &IptSupplierComponentData) -> Self {
        let IptSupplierComponentData {
            component_uuid,
            company_uuid,
            description,
            ..
        } = ipt_data;

        Self {
            component_uuid: *component_uuid,
            company_uuid: *company_uuid,
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelSupplierToComponentData {
    pub component_uuid: Uuid,
    pub companies_uuids: Vec<Uuid>,
}
