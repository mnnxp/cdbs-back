use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = actual_status_ref)]
pub(crate) struct ActualStatus {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = actual_status_ref)]
pub(crate) struct InsertableActualStatus {
    pub(crate) id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptActualStatusData {
    pub(crate) id: i32,
}

// ActualStatus translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[diesel(primary_key(actual_status_id, lang_id))]
#[diesel(belongs_to(Component, foreign_key = actual_status_id))]
#[diesel(belongs_to(ComponentModification, foreign_key = actual_status_id))]
#[diesel(belongs_to(ActualStatus, foreign_key = actual_status_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = actual_status_translate_list)]
pub(crate) struct ActualStatusTranslateList {
    pub(crate) actual_status_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptActualStatusTranslateListData {
    // pub(crate) actual_status_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = actual_status_translate_list)]
pub(crate) struct InsertableActualStatusTranslateList {
    pub(crate) actual_status_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}
