use crate::schema::*;
use crate::models::component::param::model::ComponentParam;
use crate::models::component::component_modification::param::model::ModificationParam;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

// Param models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = param_ref)]
pub(crate) struct Param {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = param_ref)]
pub(crate) struct InsertableParam {
    pub(crate) id: i32,
}

// Param translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[diesel(primary_key(param_id, lang_id))]
#[diesel(belongs_to(Param, foreign_key = param_id))]
#[diesel(belongs_to(ComponentParam, foreign_key = param_id))]
#[diesel(belongs_to(ModificationParam, foreign_key = param_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = param_translate_list)]
pub(crate) struct ParamTranslateList {
    pub(crate) param_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) paramname: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptParamTranslateListData {
    pub(crate) lang_id: i32,
    pub(crate) paramname: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = param_translate_list)]
pub(crate) struct InsertableParamTranslateList {
    pub(crate) param_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) paramname: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptParamData {
    pub(crate) param_id: i32,
    pub(crate) value: String,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptParamArg {
    pub(crate) param_ids:  Option<Vec<i32>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ParamArg {
    pub(crate) param_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
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
