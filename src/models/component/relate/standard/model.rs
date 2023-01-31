use crate::schema::*;
use crate::models::standard::model::Standard;
use crate::models::component::model::Component;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// StandardTo component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[diesel(primary_key(component_uuid, standard_uuid))]
#[diesel(belongs_to(Component, foreign_key = component_uuid))]
#[diesel(belongs_to(Standard, foreign_key = standard_uuid))]
#[diesel(table_name = standard_to_component)]
pub(crate) struct StandardToComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) standard_uuid: Uuid,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardToComponentData {
    pub(crate) standard_uuid: Uuid,
    pub(crate) component_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = standard_to_component)]
pub(crate) struct InsertableStandardToComponent {
    pub(crate) standard_uuid: Uuid,
    pub(crate) component_uuid: Uuid,
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
pub(crate) struct DelStandardToComponentData {
    pub(crate) standards_uuids: Vec<Uuid>,
    pub(crate) component_uuid: Uuid,
}
