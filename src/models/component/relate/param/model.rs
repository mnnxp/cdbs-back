use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::param::model::ParamTranslateList;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Param component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(component_uuid, param_id)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(ParamTranslateList, foreign_key = "param_id")]
#[table_name = "param_to_component"]
pub struct ParamComponent {
    pub component_uuid: Uuid,
    pub param_id: i32,
    pub value: String,
}

#[Object]
impl ParamComponent {
    async fn component_uuid(&self) -> ID {
        self.component_uuid.into()
    }
    async fn param_id(&self) -> &i32 {
        &self.param_id
    }
    async fn value(&self) -> &String {
        &self.value
    }
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub struct ComponentParamWithTranslation {
    pub component_uuid: Uuid,
    pub param: ParamTranslateList,
    pub value: String,
}

impl From<(ParamComponent, ParamTranslateList)> for ComponentParamWithTranslation {
    fn from(data: (ParamComponent, ParamTranslateList)) -> Self {
        Self {
            component_uuid: data.0.component_uuid,
            param: data.1,
            value: data.0.value,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptParamComponentData {
    pub component_uuid: Uuid,
    pub param_id: i32,
    pub value: String,
}

#[derive(Debug, Insertable)]
#[table_name = "param_to_component"]
pub struct InsertableParamComponent {
    pub component_uuid: Uuid,
    pub param_id: i32,
    pub value: String,
}

impl From<IptParamComponentData> for InsertableParamComponent {
    fn from(ipt_data: IptParamComponentData) -> Self {
        let IptParamComponentData {
            component_uuid,
            param_id,
            value,
            ..
        } = ipt_data;

        Self {
            component_uuid: Uuid::parse_str(&component_uuid.to_string()).unwrap(),
            param_id,
            value,
        }
    }
}
