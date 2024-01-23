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

/// Наименования типа компании с локализацией для заданного языка
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[diesel(primary_key(company_type_id, lang_id))]
#[diesel(belongs_to(Company, foreign_key = company_type_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = company_type_translate_list)]
pub(crate) struct CompanyTypeTranslateList {
    /// Идентификатор типа компании
    pub(crate) company_type_id: i32,
    /// Идентификатор языка локализации
    pub(crate) lang_id: i32,
    /// Полное наименование типа компании
    pub(crate) name: String,
    /// Сокращенное наименование типа компании
    pub(crate) shortname: String,
}

/// Данные для добавления новой локализации (перевода) для типа компании
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyTypeTranslateListData {
    /// Идентификатор языка локализации
    pub(crate) lang_id: i32,
    /// Полное наименование типа компании
    pub(crate) name: String,
    /// Сокращенное наименование типа компании
    pub(crate) shortname: String,
}

/// Данные для добавления нового типа компании
#[derive(Debug, Insertable)]
#[diesel(table_name = company_type_translate_list)]
pub(crate) struct InsertableCompanyTypeTranslateList {
    /// Идентификатор типа компании
    pub(crate) company_type_id: i32,
    /// Идентификатор языка локализации
    pub(crate) lang_id: i32,
    /// Полное наименование типа компании
    pub(crate) name: String,
    /// Сокращенное наименование типа компании
    pub(crate) shortname: String,
}
