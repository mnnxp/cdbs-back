use crate::schema::*;
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Param models

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Param {
    pub id: i32,
    pub paramname: String,
}

#[Object]
impl Param {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn paramname(&self) -> &String {
        &self.paramname
    }
}

#[derive(Debug, Insertable)]
#[table_name = "param_ref"]
pub struct InsertableParam {
    pub paramname: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, InputObject)]
pub struct ParamData {
    pub paramname: String,
}

impl From<Param> for ParamData {
    fn from(file: Param) -> Self {
        let Param {
            paramname,
            ..
        } = file;

        Self {
            paramname,
        }
    }
}

impl From<ParamData> for InsertableParam {
    fn from(data_param: ParamData) -> Self {
        let ParamData {
            paramname,
            ..
        } = data_param;

        Self {
            paramname,
        }
    }
}

// Param component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(uuid_component, id_param)]
#[belongs_to(Component, foreign_key = "uuid_component")]
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
