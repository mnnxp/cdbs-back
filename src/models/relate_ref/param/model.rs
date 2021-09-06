use crate::schema::*;
use crate::models::component::param::model::ParamComponent;
use crate::models::component::component_modification::param::model::ParamModification;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

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
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
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
