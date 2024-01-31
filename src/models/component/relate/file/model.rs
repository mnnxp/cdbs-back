use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Component
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[diesel(primary_key(file_uuid, component_uuid))]
#[diesel(belongs_to(ShowFileRelatedData, foreign_key = file_uuid))]
#[diesel(belongs_to(Component, foreign_key = component_uuid))]
#[diesel(table_name = file_to_component)]
pub(crate) struct ComponentFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) component_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = file_to_component)]
pub(crate) struct InsertableComponentFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) component_uuid: Uuid,
}

impl From<ComponentFile> for InsertableComponentFile {
    fn from(ipt_data: ComponentFile) -> Self {
        let ComponentFile {
            file_uuid,
            component_uuid,
            ..
        } = ipt_data;

        Self {
            file_uuid,
            component_uuid,
        }
    }
}

/// Data for the request to update the main image of the component
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentFaviconData {
    /// Name of the file to be uploaded
    pub(crate) filename: String,
    /// Component UUID
    pub(crate) component_uuid: Uuid,
}

/// Data of the request to add component files
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentFilesData {
    /// Name of files to be uploaded (list)
    pub(crate) filenames: Vec<String>,
    /// Component UUID
    pub(crate) component_uuid: Uuid,
}

/// Component file deletion request data
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct DelComponentFileData {
    /// UUID of files to be deleted (list)
    pub(crate) file_uuid: Uuid,
    /// UUID of the component
    pub(crate) component_uuid: Uuid,
}
