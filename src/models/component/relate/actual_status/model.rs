use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "actual_status_ref"]
pub struct ActualStatus {
    pub id: i32,
}

#[Object]
impl ActualStatus {
    async fn id(&self) -> &i32 {
        &self.id
    }
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
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(id_actual_status, id_lang)]
#[belongs_to(Component, foreign_key = "id_actual_status")]
#[belongs_to(ComponentModification, foreign_key = "id_actual_status")]
#[belongs_to(ActualStatus, foreign_key = "id_actual_status")]
#[belongs_to(Language, foreign_key = "id_lang")]
#[table_name = "actual_status_translate_list"]
pub struct ActualStatusTranslateList {
    pub id_actual_status: i32,
    pub id_lang: i32,
    pub name: String,
}

#[Object]
impl ActualStatusTranslateList {
    async fn id_actual_status(&self) -> &i32 {
        &self.id_actual_status
    }
    async fn id_lang(&self) -> &i32 {
        &self.id_lang
    }
    async fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptActualStatusTranslateListData {
    pub id_actual_status: i32,
    pub id_lang: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "actual_status_translate_list"]
pub struct InsertableActualStatusTranslateList {
    pub id_actual_status: i32,
    pub id_lang: i32,
    pub name: String,
}

impl From<IptActualStatusTranslateListData> for InsertableActualStatusTranslateList {
    fn from(ipt_data: IptActualStatusTranslateListData) -> Self {
        let IptActualStatusTranslateListData {
            id_actual_status,
            id_lang,
            name,
            ..
        } = ipt_data;

        Self {
            id_actual_status,
            id_lang,
            name,
        }
    }
}
