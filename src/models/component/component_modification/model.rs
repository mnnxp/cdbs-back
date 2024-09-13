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

#[derive(Identifiable, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
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

/// Full information about component (part) modification and related data
#[derive(Deserialize, SimpleObject, Debug)]
pub(crate) struct ComponentModificationAndRelatedData {
    /// UUID of the component modification
    pub(crate) uuid: Uuid,
    /// UUID of component
    pub(crate) component_uuid: Uuid,
    /// UUID of the parent modification of the component
    pub(crate) parent_modification_uuid: Uuid,
    /// Name of the component modification
    pub(crate) modification_name: String,
    /// Description of the component modification
    pub(crate) description: String,
    /// Current status of the component modification
    pub(crate) actual_status: ActualStatusTranslateList,
    /// Date of creation of the component modification
    pub(crate) created_at: NaiveDateTime,
    /// Date when the main data of the component modification was changed
    pub(crate) updated_at: NaiveDateTime,
    /// Component modification file sets data (list)
    pub(crate) filesets_for_program: Vec<FilesetProgramRelatedData>,
    /// Data on component modification parameters (list)
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
    /// Returns a structure with the specified component UUID
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

    /// Returns structures with the specified component UUID and modifications data
    pub(crate) fn get_multiple_data(data: &IptMultipleModificationsData) -> Vec<Self> {
        let mut res = Vec::new();
        for md in data.modifications_data.iter() {
            let local_time = chrono::Local::now().naive_local();
            res.push(Self {
                uuid: Uuid::new_v4(),
                component_uuid: data.component_uuid,
                parent_modification_uuid: Uuid::nil(),
                modification_name: md.modification_name.clone(),
                description: md.description.clone(),
                actual_status_id: md.actual_status_id,
                is_delete: false,
                created_at: local_time,
                updated_at: local_time,
            })
        }
        res
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

/// Data for adding a new modification to a component
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptComponentModificationData {
    /// UUID of the component to which the modification will be added
    pub(crate) component_uuid: Uuid,
    /// UUID of the parent modification of the component (optional)
    pub(crate) parent_modification_uuid: Option<Uuid>,
    /// Name of the component modification
    pub(crate) modification_name: String,
    /// Description of the component modification
    pub(crate) description: String,
    /// Current status of the component modification
    pub(crate) actual_status_id: i32,
}

/// Data for adding multiple modifications to a component
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptMultipleModificationsData {
    /// UUID of the component to which modifications will be added
    pub(crate) component_uuid: Uuid,
    /// Basic data of new modifications
    pub(crate) modifications_data: Vec<IptModificationsData>,
}

/// Data for adding multiple modifications to a component
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptModificationsData {
    /// Name of the component modification
    pub(crate) modification_name: String,
    /// Description of the component modification
    pub(crate) description: String,
    /// Current status of the component modification
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

/// Structure for updating the basic data of a component modification
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateComponentModificationData {
    /// New name of the component modification (optional)
    pub(crate) modification_name: Option<String>,
    /// New description of the component modification (optional)
    pub(crate) description: Option<String>,
    /// Update the status of the component modification (optional)
    pub(crate) actual_status_id: Option<i32>,
}

/// Component modification deletion request data
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelComponentModificationData {
    /// UUID of the component to which the component modification applies
    pub(crate) component_uuid: Uuid,
    /// UUID of the component modification to be deleted
    pub(crate) modification_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentModificationArg {
    pub(crate) component_uuid: Uuid,
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

/// Component modification file list request data
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptModificationFilesArg {
    /// UUID of component modification
    pub(crate) modification_uuid: Uuid,
    /// Filtering files by UUID (list)
    pub(crate) files_uuids: Option<Vec<Uuid>>,
    /// Restriction of data sampling (maximum number of records)
    pub(crate) limit: Option<i32>,
    /// Number of skipping records at the beginning (offset)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ModificationFilesArg {
    pub(crate) modification_uuid: Uuid,
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
