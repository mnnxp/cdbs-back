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
    pub id_lang: i32,
    pub paramname: String,
}

#[Object]
impl Param {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn id_lang(&self) -> &i32 {
        &self.id_lang
    }
    async fn paramname(&self) -> &String {
        &self.paramname
    }
}

#[derive(Debug, Insertable)]
#[table_name = "param_ref"]
pub struct InsertableParam {
    pub id_lang: i32,
    pub paramname: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, InputObject)]
pub struct ParamData {
    pub id_lang: i32,
    pub paramname: String,
}

impl From<Param> for ParamData {
    fn from(data: Param) -> Self {
        let Param {
            id_lang,
            paramname,
            ..
        } = data;

        Self {
            id_lang,
            paramname,
        }
    }
}

impl From<ParamData> for InsertableParam {
    fn from(ipt_data: ParamData) -> Self {
        let ParamData {
            id_lang,
            paramname,
            ..
        } = ipt_data;

        Self {
            id_lang,
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
