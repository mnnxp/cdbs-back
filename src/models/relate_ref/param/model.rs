use crate::schema::*;
use async_graphql::*;
use diesel::sql_types;

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
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug, Default)]
#[diesel(table_name = param_translate_list)]
pub(crate) struct ParamTranslateList {
    /// Parameter identifier
    pub(crate) param_id: i32,
    /// Name localization language identifier
    pub(crate) lang_id: i32,
    /// Localized name of the parameter
    pub(crate) paramname: String,
}

/// Structure for parameter value queries
#[derive(QueryableByName)]
pub(crate) struct ParamValue {
    /// Parameter identifier
    #[diesel(sql_type = sql_types::Integer)]
    pub(crate) param_id: i32,
    /// Value of the service parameter
    #[diesel(sql_type = sql_types::Text)]
    pub(crate) value: String,
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
