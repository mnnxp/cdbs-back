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

/// Сведения о поставщике компонента с описанием
#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub(crate) struct ComponentSupplierRelatedData {
    /// Данные о компании-поставщике
    pub(crate) supplier: SlimCompany,
    /// UUID компонента
    pub(crate) component_uuid: Uuid,
    /// Описание поставщика для данного компонента
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

/// Данные для запросов на добавление основого поставщика
/// и добавления компании в список поставщиков компонента
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptSupplierComponentData {
    /// UUID компонента
    pub(crate) component_uuid: Uuid,
    /// UUID компании
    pub(crate) company_uuid: Uuid,
    /// Описание поставщика (примечание к компоненту от поставщика)
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

/// Данные для запроса на уделение поставщиков компонента
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelSuppliersComponentData {
    /// UUID компонента
    pub(crate) component_uuid: Uuid,
    /// UUIDs компаний-поставщиков подлежащих удалению
    pub(crate) companies_uuids: Vec<Uuid>,
}
