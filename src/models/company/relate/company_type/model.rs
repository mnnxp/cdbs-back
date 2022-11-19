use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::relate_ref::language::model::Language;
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

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyTypeData {
    pub(crate) id: i32,
}

// CompanyType translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[diesel(primary_key(company_type_id, lang_id))]
#[diesel(belongs_to(Company, foreign_key = company_type_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = company_type_translate_list)]
pub(crate) struct CompanyTypeTranslateList {
    pub(crate) company_type_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
    pub(crate) shortname: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyTypeTranslateListData {
    pub(crate) lang_id: i32,
    pub(crate) name: String,
    pub(crate) shortname: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_type_translate_list)]
pub(crate) struct InsertableCompanyTypeTranslateList {
    pub(crate) company_type_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
    pub(crate) shortname: String,
}
