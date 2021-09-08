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
#[primary_key(id_standard_status, id_lang)]
#[belongs_to(Standard, foreign_key = "id_standard_status")]
#[belongs_to(Language, foreign_key = "id_lang")]
#[table_name = "standard_status_translate_list"]
pub struct StandardStatusTranslateList {
    pub id_standard_status: i32,
    pub id_lang: i32,
    pub name: String,
}

#[Object]
impl StandardStatusTranslateList {
    async fn id_standard_status(&self) -> &i32 {
        &self.id_standard_status
    }
    async fn id_lang(&self) -> &i32 {
        &self.id_lang
    }
    async fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptStandardStatusTranslateListData {
    pub id_lang: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "standard_status_translate_list"]
pub struct InsertableStandardStatusTranslateList {
    pub id_standard_status: i32,
    pub id_lang: i32,
    pub name: String,
}
