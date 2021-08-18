use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::component::component_modification::param::model::ParamModification;
use crate::models::language::model::Language;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Param models

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "param_ref"]
pub struct Param {
    pub id: i32,
}

#[Object]
impl Param {
    async fn id(&self) -> &i32 {
        &self.id
    }
}

#[derive(Debug, Insertable)]
#[table_name = "param_ref"]
pub struct InsertableParam {
    pub id: i32,
}

// Param translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(id_param, id_lang)]
#[belongs_to(Param, foreign_key = "id_param")]
#[belongs_to(ParamComponent, foreign_key = "id_param")]
#[belongs_to(ParamModification, foreign_key = "id_param")]
#[belongs_to(Language, foreign_key = "id_lang")]
#[table_name = "param_translate_list"]
pub struct ParamTranslateList {
    pub id_param: i32,
    pub id_lang: i32,
    pub paramname: String,
}

#[Object]
impl ParamTranslateList {
    async fn id_param(&self) -> &i32 {
        &self.id_param
    }
    async fn id_lang(&self) -> &i32 {
        &self.id_lang
    }
    async fn paramname(&self) -> &String {
        &self.paramname
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptParamTranslateListData {
    pub id_lang: i32,
    pub paramname: String,
}

#[derive(Debug, Insertable)]
#[table_name = "param_translate_list"]
pub struct InsertableParamTranslateList {
    pub id_param: i32,
    pub id_lang: i32,
    pub paramname: String,
}

// Param component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
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

#[derive(Debug, Deserialize, SimpleObject, Description)]
pub struct ParamComponentRelate {
    pub uuid_component: Uuid,
    pub param: ParamTranslateList,
    pub value: String,
}

impl From<(ParamComponent, ParamTranslateList)> for ParamComponentRelate {
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
