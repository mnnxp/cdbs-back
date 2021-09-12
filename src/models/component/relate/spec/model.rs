use crate::schema::*;
use crate::models::relate_ref::spec::model::{Spec, SpecTranslateList};
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Spec component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(component_uuid, spec_id)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(Spec, foreign_key = "spec_id")]
#[table_name = "spec_to_component"]
pub struct SpecComponent {
    pub spec_id: i32,
    pub component_uuid: Uuid,
}

#[Object]
impl SpecComponent {
    async fn spec_id(&self) -> &i32 {
        &self.spec_id
    }
    async fn component_uuid(&self) -> ID {
        self.component_uuid.into()
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct ComponentSpecWithTranslation {
    pub spec: SpecTranslateList,
    pub component_uuid: Uuid,
}

impl From<(SpecComponent, SpecTranslateList)> for ComponentSpecWithTranslation {
    fn from(data: (SpecComponent, SpecTranslateList)) -> Self {
        Self {
            spec: data.1,
            component_uuid: data.0.component_uuid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSpecComponentData {
    pub component_uuid: ID,
    pub spec_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_to_component"]
pub struct InsertableSpecComponent {
    pub component_uuid: Uuid,
    pub spec_id: i32,
}

impl From<IptSpecComponentData> for InsertableSpecComponent {
    fn from(ipt_data: IptSpecComponentData) -> Self {
        let IptSpecComponentData {
            component_uuid,
            spec_id,
            ..
        } = ipt_data;

        Self {
            component_uuid: Uuid::parse_str(&component_uuid.to_string()).unwrap(),
            spec_id,
        }
    }
}
