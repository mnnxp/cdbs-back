use crate::models::component::relate::actual_status::model::ActualStatusTranslateList;
use crate::models::component::component_modification::param::model::ModificationParamWithTranslation;
use crate::models::component::component_modification::fileset_for_program::model::FilesetProgramRelatedData;
use crate::models::component::model::Component;
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Associations, PartialEq, Clone, SimpleObject, Debug)]
#[primary_key(uuid)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[table_name = "component_modification_list"]
pub struct ComponentModification {
    pub uuid: Uuid,
    pub component_uuid: Uuid,
    pub parent_modification_uuid: Uuid,
    pub modification_name: String,
    pub description: String,
    pub actual_status_id: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, SimpleObject, Debug)]
pub struct ComponentModificationAndRelatedData {
    pub uuid: Uuid,
    pub component_uuid: Uuid,
    pub parent_modification_uuid: Uuid,
    pub modification_name: String,
    pub description: String,
    pub actual_status: ActualStatusTranslateList,
    // pub actual_status_id: i32,
    pub updated_at: NaiveDateTime,
    pub filesets_for_program: Vec<FilesetProgramRelatedData>,
    pub modification_params: Vec<ModificationParamWithTranslation>,
}

impl From<(
    ComponentModificationWithActualStatus,
    Vec<FilesetProgramRelatedData>,
    Vec<ModificationParamWithTranslation>
)> for ComponentModificationAndRelatedData {
    fn from(data: (
        ComponentModificationWithActualStatus,
        Vec<FilesetProgramRelatedData>,
        Vec<ModificationParamWithTranslation>
    )) -> Self {
        Self {
            uuid: data.0.modification.uuid,
            component_uuid: data.0.modification.component_uuid,
            parent_modification_uuid: data.0.modification.parent_modification_uuid,
            modification_name: data.0.modification.modification_name,
            description: data.0.modification.description,
            actual_status: data.0.actual_status,
            // actual_status_id: data.0.modification.actual_status_id,
            updated_at: data.0.modification.updated_at,
            filesets_for_program: data.1,
            modification_params: data.2,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComponentModificationWithActualStatus {
    pub modification: ComponentModification,
    pub actual_status: ActualStatusTranslateList,
}

impl From<(ComponentModification, ActualStatusTranslateList)> for ComponentModificationWithActualStatus {
    fn from(data: (ComponentModification, ActualStatusTranslateList)) -> Self {
        Self {
            modification: data.0,
            actual_status: data.1,
        }
    }
}


#[derive(Debug, Insertable)]
#[table_name = "component_modification_list"]
pub struct InsertableComponentModification {
    pub uuid: Uuid,
    pub component_uuid: Uuid,
    pub parent_modification_uuid: Uuid,
    pub modification_name: String,
    pub description: String,
    pub actual_status_id: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InsertableComponentModification {
    /// Change parent uuid to base for insert new row
    pub(crate) fn parent_uuid_to_base(&mut self) {
        self.parent_modification_uuid = Uuid::parse_str("aba22d59-4f6c-44a4-9a37-2d38f0e577a8").unwrap();
    }

    // /// Change parent uuid for insert new row
    // pub(crate) fn change_parent_uuid(&mut self, new_uuid: &Uuid) {
    //     self.parent_modification_uuid = *new_uuid;
    // }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentModificationData {
    pub component_uuid: Uuid,
    pub parent_modification_uuid: Option<Uuid>,
    pub modification_name: String,
    pub description: String,
    pub actual_status_id: i32,
}

impl From<&IptComponentModificationData> for InsertableComponentModification {
    fn from(ipt_data: &IptComponentModificationData) -> Self {
        let IptComponentModificationData {
            component_uuid,
            parent_modification_uuid,
            modification_name,
            description,
            actual_status_id,
        } = ipt_data;

        let uuid = Uuid::new_v4();
        let parent_modification_uuid = match parent_modification_uuid {
            Some(parent_uuid) => *parent_uuid,
            None => uuid,
        };

        Self {
            uuid,
            component_uuid: *component_uuid,
            parent_modification_uuid,
            modification_name: modification_name.to_string(),
            description: description.to_string(),
            actual_status_id: *actual_status_id,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUpdateComponentModificationData {
    // pub parent_modification_uuid: Option<Uuid>,
    pub modification_name: Option<String>,
    pub description: Option<String>,
    pub actual_status_id: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelComponentModificationData {
    pub component_uuid: Uuid,
    pub modification_uuid: Uuid,
}
