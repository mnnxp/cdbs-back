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
/// Localized parameter data. Parameters are used as a characterization element
/// to add characteristics to components, component modifications, and standards
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[diesel(primary_key(param_id, lang_id))]
#[diesel(belongs_to(Param, foreign_key = param_id))]
#[diesel(belongs_to(ComponentParam, foreign_key = param_id))]
#[diesel(belongs_to(ModificationParam, foreign_key = param_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = param_translate_list)]
pub(crate) struct ParamTranslateList {
    /// Parameter identifier
    pub(crate) param_id: i32,
    /// Name localization language identifier
    pub(crate) lang_id: i32,
    /// Localized name of the parameter
    pub(crate) paramname: String,
}

/// Data for a request to add a new parameter
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptParamTranslateListData {
    /// Localization language identifier
    pub(crate) lang_id: i32,
    /// Localized name of the parameter
    pub(crate) paramname: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = param_translate_list)]
pub(crate) struct InsertableParamTranslateList {
    pub(crate) param_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) paramname: String,
}

/// Data for requests to add a parameter value
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptParamData {
    /// Parameter ID
    pub(crate) param_id: i32,
    /// Parameter value
    pub(crate) value: String,
}