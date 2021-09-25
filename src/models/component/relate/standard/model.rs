use crate::schema::*;
use crate::models::standard::model::Standard;
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// StandardTo component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(component_uuid, standard_uuid)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(Standard, foreign_key = "standard_uuid")]
#[table_name = "standard_to_component"]
pub struct StandardToComponent {
    pub component_uuid: Uuid,
    pub standard_uuid: Uuid,
}

#[Object]
impl StandardToComponent {
    async fn standard_uuid(&self) -> ID {
        self.standard_uuid.into()
    }
    async fn component_uuid(&self) -> ID {
        self.component_uuid.into()
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptStandardToComponentData {
    pub standard_uuid: Uuid,
    pub component_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "standard_to_component"]
pub struct InsertableStandardToComponent {
    pub standard_uuid: Uuid,
    pub component_uuid: Uuid,
}

impl From<&IptStandardToComponentData> for InsertableStandardToComponent {
    fn from(ipt_data: &IptStandardToComponentData) -> Self {
        let IptStandardToComponentData {
            standard_uuid,
            component_uuid,
            ..
        } = ipt_data;

        Self {
            standard_uuid: *standard_uuid,
            component_uuid: *component_uuid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelStandardToComponentData {
    pub standards_uuids: Vec<Uuid>,
    pub component_uuid: Uuid,
}
