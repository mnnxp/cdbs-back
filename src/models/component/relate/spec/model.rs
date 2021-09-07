use crate::schema::*;
use crate::models::relate_ref::spec::model::{Spec, SpecTranslateList};
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Spec component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_component, id_spec)]
#[belongs_to(Component, foreign_key = "uuid_component")]
#[belongs_to(Spec, foreign_key = "id_spec")]
#[table_name = "spec_to_component"]
pub struct SpecComponent {
    pub id_spec: i32,
    pub uuid_component: Uuid,
}

#[Object]
impl SpecComponent {
    async fn id_spec(&self) -> &i32 {
        &self.id_spec
    }
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct ComponentSpecWithTranslation {
    pub spec: SpecTranslateList,
    pub uuid_component: Uuid,
}

impl From<(SpecComponent, SpecTranslateList)> for ComponentSpecWithTranslation {
    fn from(data: (SpecComponent, SpecTranslateList)) -> Self {
        Self {
            spec: data.1,
            uuid_component: data.0.uuid_component,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSpecComponentData {
    pub uuid_component: ID,
    pub id_spec: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_to_component"]
pub struct InsertableSpecComponent {
    pub uuid_component: Uuid,
    pub id_spec: i32,
}

impl From<IptSpecComponentData> for InsertableSpecComponent {
    fn from(ipt_data: IptSpecComponentData) -> Self {
        let IptSpecComponentData {
            uuid_component,
            id_spec,
            ..
        } = ipt_data;

        Self {
            uuid_component: Uuid::parse_str(&uuid_component.to_string()).unwrap(),
            id_spec,
        }
    }
}
