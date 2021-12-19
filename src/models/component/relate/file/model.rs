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
pub struct ComponentFile {
    pub file_uuid: Uuid,
    pub component_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_component"]
pub struct InsertableComponentFile {
    pub file_uuid: Uuid,
    pub component_uuid: Uuid,
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
pub struct IptComponentFileData {
    pub filenames: Vec<String>,
    pub component_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub struct DelComponentFileData {
    pub file_uuid: Uuid,
    pub component_uuid: Uuid,
}
