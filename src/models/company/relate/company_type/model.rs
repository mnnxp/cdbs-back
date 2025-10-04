use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = company_type_ref)]
pub(crate) struct CompanyType {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_type_ref)]
pub(crate) struct InsertableCompanyType {
    pub(crate) id: i32,
}

/// Company type names with localization for the specified language
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = company_type_translate_list)]
pub(crate) struct CompanyTypeTranslateList {
    /// Company type identifier
    pub(crate) company_type_id: i32,
    /// Localization language identifier
    pub(crate) lang_id: i32,
    /// Full name of the company type
    pub(crate) name: String,
    /// Abbreviated name of the company type
    pub(crate) shortname: String,
}

/// Data for adding a new localization (translation) for the company type
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyTypeTranslateListData {
    /// Localization language identifier
    pub(crate) lang_id: i32,
    /// Full name of the company type
    pub(crate) name: String,
    /// Abbreviated name of the company type
    pub(crate) shortname: String,
}

/// Data for adding a new company type
#[derive(Debug, Insertable)]
#[diesel(table_name = company_type_translate_list)]
pub(crate) struct InsertableCompanyTypeTranslateList {
    /// Company type identifier
    pub(crate) company_type_id: i32,
    /// Localization language identifier
    pub(crate) lang_id: i32,
    /// Full name of the company type
    pub(crate) name: String,
    /// Abbreviated name of the company type
    pub(crate) shortname: String,
}
