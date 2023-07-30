use crate::models::component::{
    model::Component,
    relate::actual_status::model::ActualStatusTranslateList,
};
use crate::models::component::component_modification::{
    param::model::ModificationParamWithTranslation,
    fileset_for_program::model::FilesetProgramRelatedData,
    util::get_root_modification_uuid,
};
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Associations, PartialEq, Clone, SimpleObject, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(belongs_to(Component, foreign_key = component_uuid))]
#[diesel(table_name = component_modification_list)]
pub(crate) struct ComponentModification {
    pub(crate) uuid: Uuid,
    pub(crate) component_uuid: Uuid,
    pub(crate) parent_modification_uuid: Uuid,
    pub(crate) modification_name: String,
    pub(crate) description: String,
    pub(crate) actual_status_id: i32,
    pub(crate) is_delete: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Deserialize, SimpleObject, Debug)]
pub(crate) struct ComponentModificationAndRelatedData {
    pub(crate) uuid: Uuid,
    pub(crate) component_uuid: Uuid,
    pub(crate) parent_modification_uuid: Uuid,
    pub(crate) modification_name: String,
    pub(crate) description: String,
    pub(crate) actual_status: ActualStatusTranslateList,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
    pub(crate) filesets_for_program: Vec<FilesetProgramRelatedData>,
    pub(crate) modification_params: Vec<ModificationParamWithTranslation>,
}

impl ComponentModificationAndRelatedData {
    /// Create struct with data ComponentModification, set default data for related data
    pub(crate) fn new(data: &ComponentModification) -> Self {
        Self{
            uuid: data.uuid,
            component_uuid: data.component_uuid,
            parent_modification_uuid: data.parent_modification_uuid,
            modification_name: data.modification_name.clone(),
            description: data.description.clone(),
            actual_status: Default::default(),
            created_at: data.created_at,
            updated_at: data.updated_at,
            filesets_for_program: Vec::new(),
            modification_params: Vec::new(),
        }
    }

    /// Change actual satus data
    pub(crate) fn put_actual_status(&mut self, actual_status: &ActualStatusTranslateList) {
        self.actual_status = actual_status.clone();
    }

    /// Change filesets data
    pub(crate) fn put_fileset_program(&mut self, fileset: Vec<FilesetProgramRelatedData>) {
        self.filesets_for_program = fileset;
    }

    /// Change modification params
    pub(crate) fn put_modification_params(&mut self, params: Vec<ModificationParamWithTranslation>) {
        self.modification_params = params;
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = component_modification_list)]
pub(crate) struct InsertableComponentModification {
    uuid: Uuid,
    component_uuid: Uuid,
    parent_modification_uuid: Uuid,
    modification_name: String,
    description: String,
    actual_status_id: i32,
    is_delete: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl InsertableComponentModification {
    /// Check parent modification uuid on nil
    pub(crate) fn get_default_for_component(component_uuid: &Uuid) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            component_uuid: *component_uuid,
            parent_modification_uuid: Uuid::nil(),
            modification_name: "N1".to_string(),
            description: String::new(),
            actual_status_id: 1,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    /// Check parent modification uuid on nil
    pub(crate) fn parent_uuid_is_nil(&self) -> bool {
        self.parent_modification_uuid.is_nil()
    }

    /// Change parent uuid to base for insert new row
    pub(crate) fn parent_uuid_to_base(&mut self) {
        self.parent_modification_uuid = get_root_modification_uuid();
    }
}

#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptComponentModificationData {
    pub(crate) component_uuid: Uuid,
    pub(crate) parent_modification_uuid: Option<Uuid>,
    pub(crate) modification_name: String,
    pub(crate) description: String,
    pub(crate) actual_status_id: i32,
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

        let parent_modification_uuid = match parent_modification_uuid {
            Some(parent_uuid) => *parent_uuid,
            None => Uuid::nil(),
        };

        Self {
            uuid: Uuid::new_v4(),
            component_uuid: *component_uuid,
            parent_modification_uuid,
            modification_name: modification_name.clone(),
            description: description.clone(),
            actual_status_id: *actual_status_id,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateComponentModificationData {
    // pub(crate) parent_modification_uuid: Option<Uuid>,
    pub(crate) modification_name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) actual_status_id: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelComponentModificationData {
    pub(crate) component_uuid: Uuid,
    pub(crate) modification_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentModificationArg {
    pub(crate) component_uuid:  Uuid,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ComponentModificationArg {
    pub(crate) component_uuid: Uuid,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl ComponentModificationArg {
    /// Generate default limit 100 and offset 0
    pub(crate) fn component_uuid(component_uuid: &Uuid) -> Self {
        Self {
            component_uuid: *component_uuid,
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptComponentModificationArg> for ComponentModificationArg {
    fn from(data: IptComponentModificationArg) -> Self {
        let IptComponentModificationArg {
            component_uuid,
            limit,
            offset,
        } = data;

        Self {
            component_uuid,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptModificationFilesArg {
    pub(crate) modification_uuid:  Uuid,
    pub(crate) files_uuids: Option<Vec<Uuid>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ModificationFilesArg {
    pub(crate) modification_uuid:  Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl From<IptModificationFilesArg> for ModificationFilesArg {
    fn from(data: IptModificationFilesArg) -> Self {
        let IptModificationFilesArg {
            modification_uuid,
            files_uuids,
            limit,
            offset,
        } = data;

        Self {
            modification_uuid,
            file_uuids: files_uuids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
