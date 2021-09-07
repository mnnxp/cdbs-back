use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::param::model::ParamTranslateList;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Param component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_component, id_param)]
#[belongs_to(Component, foreign_key = "uuid_component")]
#[belongs_to(ParamTranslateList, foreign_key = "id_param")]
#[table_name = "param_to_component"]
pub struct ParamComponent {
    pub uuid_component: Uuid,
    pub id_param: i32,
    pub value: String,
}

#[Object]
impl ParamComponent {
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn id_param(&self) -> &i32 {
        &self.id_param
    }
    async fn value(&self) -> &String {
        &self.value
    }
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub struct ComponentParamWithTranslation {
    pub uuid_component: Uuid,
    pub param: ParamTranslateList,
    pub value: String,
}

impl From<(ParamComponent, ParamTranslateList)> for ComponentParamWithTranslation {
    fn from(data: (ParamComponent, ParamTranslateList)) -> Self {
        Self {
            uuid_component: data.0.uuid_component,
            param: data.1,
            value: data.0.value,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptParamComponentData {
    pub uuid_component: ID,
    pub id_param: i32,
    pub value: String,
}

#[derive(Debug, Insertable)]
#[table_name = "param_to_component"]
pub struct InsertableParamComponent {
    pub uuid_component: Uuid,
    pub id_param: i32,
    pub value: String,
}

impl From<IptParamComponentData> for InsertableParamComponent {
    fn from(ipt_data: IptParamComponentData) -> Self {
        let IptParamComponentData {
            uuid_component,
            id_param,
            value,
            ..
        } = ipt_data;

        Self {
            uuid_component: Uuid::parse_str(&uuid_component.to_string()).unwrap(),
            id_param,
            value,
        }
    }
}
