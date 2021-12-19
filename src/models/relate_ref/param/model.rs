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

#[derive(Debug, Insertable)]
#[table_name = "param_ref"]
pub struct InsertableParam {
    pub id: i32,
}

// Param translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
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

#[derive(InputObject, Deserialize, Debug)]
pub struct IptParamArg {
    pub param_ids:  Option<Vec<i32>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct ParamArg {
    pub param_ids: Vec<i32>,
    pub limit: i32,
    pub offset: i32,
}

impl Default for ParamArg {
    fn default() -> Self {
        Self {
            param_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptParamArg> for ParamArg {
    fn from(data: IptParamArg) -> Self {
        let IptParamArg {
            param_ids,
            limit,
            offset,
        } = data;

        Self {
            param_ids: param_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
