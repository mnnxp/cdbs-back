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

#[Object]
impl CompanyType {
    async fn id(&self) -> &i32 {
        &self.id
    }
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
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(id_company_type, id_lang)]
#[belongs_to(Company, foreign_key = "id_company_type")]
#[belongs_to(Language, foreign_key = "id_lang")]
#[table_name = "company_type_translate_list"]
pub struct CompanyTypeTranslateList {
    pub id_company_type: i32,
    pub id_lang: i32,
    pub name: String,
    pub shortname: String,
}

#[Object]
impl CompanyTypeTranslateList {
    async fn id_company_type(&self) -> &i32 {
        &self.id_company_type
    }
    async fn id_lang(&self) -> &i32 {
        &self.id_lang
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn shortname(&self) -> &String {
        &self.shortname
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptCompanyTypeTranslateListData {
    pub id_lang: i32,
    pub name: String,
    pub shortname: String,
}

#[derive(Debug, Insertable)]
#[table_name = "company_type_translate_list"]
pub struct InsertableCompanyTypeTranslateList {
    pub id_company_type: i32,
    pub id_lang: i32,
    pub name: String,
    pub shortname: String,
}
