use crate::schema::*;
use crate::models::standard::model::Standard;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "standard_status_ref"]
pub struct StandardStatus {
    pub id: i32,
}

#[Object]
impl StandardStatus {
    async fn id(&self) -> &i32 {
        &self.id
    }
}

#[derive(Debug, Insertable)]
#[table_name = "standard_status_ref"]
pub struct InsertableStandardStatus {
    pub id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptStandardStatusData {
    pub id: i32,
}

// StandardStatus translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(standard_status_id, lang_id)]
#[belongs_to(Standard, foreign_key = "standard_status_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "standard_status_translate_list"]
pub struct StandardStatusTranslateList {
    pub standard_status_id: i32,
    pub lang_id: i32,
    pub name: String,
}

#[Object]
impl StandardStatusTranslateList {
    async fn standard_status_id(&self) -> &i32 {
        &self.standard_status_id
    }
    async fn lang_id(&self) -> &i32 {
        &self.lang_id
    }
    async fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptStandardStatusTranslateListData {
    pub lang_id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "standard_status_translate_list"]
pub struct InsertableStandardStatusTranslateList {
    pub standard_status_id: i32,
    pub lang_id: i32,
    pub name: String,
}
