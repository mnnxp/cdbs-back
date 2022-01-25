use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Component
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, SimpleObject, Debug)]
#[primary_key(file_uuid, component_uuid)]
#[belongs_to(ShowFileRelatedData, foreign_key = "file_uuid")]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[table_name = "file_to_component"]
pub(crate) struct ComponentFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) component_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_component"]
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentFaviconData {
    pub(crate) filename: String,
    pub(crate) component_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentFilesData {
    pub(crate) filenames: Vec<String>,
    pub(crate) component_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct DelComponentFileData {
    pub(crate) file_uuid: Uuid,
    pub(crate) component_uuid: Uuid,
}
