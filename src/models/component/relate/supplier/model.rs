use crate::schema::*;
use crate::models::company::model::{Company, SlimCompany};
use crate::models::component::model::Component;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Supplier component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[primary_key(component_uuid, company_uuid)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(Company, foreign_key = "company_uuid")]
#[table_name = "supplier_to_component"]
pub struct SupplierComponent {
    pub component_uuid: Uuid,
    pub company_uuid: Uuid,
    pub description: String,
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct ComponentSupplierRelatedData {
    pub supplier: SlimCompany,
    pub component_uuid: Uuid,
    pub description: String,
}

impl ComponentSupplierRelatedData {
    /// Create struct with SupplierComponent data, SlimCompany data set default
    pub(crate) fn new(data: &SupplierComponent) -> Self {
        Self{
            supplier: Default::default(),
            component_uuid: data.component_uuid,
            description: data.description.clone(),
        }
    }

    /// Change suplier data
    pub(crate) fn put_supplier(&mut self, supplier: SlimCompany) {
        self.supplier = supplier;
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
pub struct DelSuppliersComponentData {
    pub component_uuid: Uuid,
    pub companies_uuids: Vec<Uuid>,
}
