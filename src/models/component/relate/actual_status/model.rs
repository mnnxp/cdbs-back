use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, SimpleObject, Debug)]
#[primary_key(id)]
#[table_name = "actual_status_ref"]
pub struct ActualStatus {
    pub id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "actual_status_ref"]
pub struct InsertableActualStatus {
    pub id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptActualStatusData {
    pub id: i32,
}

// ActualStatus translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[primary_key(actual_status_id, lang_id)]
#[belongs_to(Component, foreign_key = "actual_status_id")]
#[belongs_to(ComponentModification, foreign_key = "actual_status_id")]
#[belongs_to(ActualStatus, foreign_key = "actual_status_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "actual_status_translate_list"]
pub struct ActualStatusTranslateList {
    pub actual_status_id: i32,
    pub lang_id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptActualStatusTranslateListData {
    // pub actual_status_id: i32,
    pub lang_id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "actual_status_translate_list"]
pub struct InsertableActualStatusTranslateList {
    pub actual_status_id: i32,
    pub lang_id: i32,
    pub name: String,
}
