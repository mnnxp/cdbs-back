use crate::schema::*;
use crate::models::standard::model::Standard;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "standard_status_ref"]
pub(crate) struct StandardStatus {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "standard_status_ref"]
pub(crate) struct InsertableStandardStatus {
    pub(crate) id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardStatusData {
    pub(crate) id: i32,
}

// StandardStatus translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[primary_key(standard_status_id, lang_id)]
#[belongs_to(Standard, foreign_key = "standard_status_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "standard_status_translate_list"]
pub(crate) struct StandardStatusTranslateList {
    pub(crate) standard_status_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardStatusTranslateListData {
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "standard_status_translate_list"]
pub(crate) struct InsertableStandardStatusTranslateList {
    pub(crate) standard_status_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}
