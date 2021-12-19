use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "company_type_ref"]
pub struct CompanyType {
    pub id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "company_type_ref"]
pub struct InsertableCompanyType {
    pub id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptCompanyTypeData {
    pub id: i32,
}

// CompanyType translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[primary_key(company_type_id, lang_id)]
#[belongs_to(Company, foreign_key = "company_type_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "company_type_translate_list"]
pub struct CompanyTypeTranslateList {
    pub company_type_id: i32,
    pub lang_id: i32,
    pub name: String,
    pub shortname: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptCompanyTypeTranslateListData {
    pub lang_id: i32,
    pub name: String,
    pub shortname: String,
}

#[derive(Debug, Insertable)]
#[table_name = "company_type_translate_list"]
pub struct InsertableCompanyTypeTranslateList {
    pub company_type_id: i32,
    pub lang_id: i32,
    pub name: String,
    pub shortname: String,
}
