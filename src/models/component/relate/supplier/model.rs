use crate::schema::*;
use crate::models::company::model::{Company, SlimCompany};
use crate::models::component::model::Component;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Supplier component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[diesel(primary_key(component_uuid, company_uuid))]
#[diesel(belongs_to(Component, foreign_key = component_uuid))]
#[diesel(belongs_to(Company, foreign_key = company_uuid))]
#[diesel(table_name = supplier_to_component)]
pub(crate) struct SupplierComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) description: String,
}

/// Component supplier information with description
#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub(crate) struct ComponentSupplierRelatedData {
    /// Data about the supplier company
    pub(crate) supplier: SlimCompany,
    /// Component UUID
    pub(crate) component_uuid: Uuid,
    /// Description of the supplier for this component
    pub(crate) description: String,
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

/// Data for requests to add the main supplier
/// and add the company to the list of suppliers of the component (part)
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptSupplierComponentData {
    /// Component UUID
    pub(crate) component_uuid: Uuid,
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// Vendor description (note to the component from the vendor)
    pub(crate) description: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = supplier_to_component)]
pub(crate) struct InsertableSupplierComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) description: String,
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

/// Data for the component's vendor assignment request
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelSuppliersComponentData {
    /// UUID of the component
    pub(crate) component_uuid: Uuid,
    /// UUIDs of supplier companies to be deleted
    pub(crate) companies_uuids: Vec<Uuid>,
}
