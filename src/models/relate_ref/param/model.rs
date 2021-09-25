use crate::schema::*;
use crate::models::component::param::model::ComponentParam;
use crate::models::component::component_modification::param::model::ModificationParam;
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
#[primary_key(param_id, lang_id)]
#[belongs_to(Param, foreign_key = "param_id")]
#[belongs_to(ComponentParam, foreign_key = "param_id")]
#[belongs_to(ModificationParam, foreign_key = "param_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "param_translate_list"]
pub struct ParamTranslateList {
    pub param_id: i32,
    pub lang_id: i32,
    pub paramname: String,
}

#[Object]
impl ParamTranslateList {
    async fn param_id(&self) -> &i32 {
        &self.param_id
    }
    async fn lang_id(&self) -> &i32 {
        &self.lang_id
    }
    async fn paramname(&self) -> &String {
        &self.paramname
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptParamTranslateListData {
    pub lang_id: i32,
    pub paramname: String,
}

#[derive(Debug, Insertable)]
#[table_name = "param_translate_list"]
pub struct InsertableParamTranslateList {
    pub param_id: i32,
    pub lang_id: i32,
    pub paramname: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptParamData {
    pub param_id: i32,
    pub value: String,
}
