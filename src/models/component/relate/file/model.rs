use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::file::model::ShowFile;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Component
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(file_uuid, component_uuid)]
#[belongs_to(ShowFile, foreign_key = "file_uuid")]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[table_name = "file_to_component"]
pub struct FileComponent {
    pub file_uuid: Uuid,
    pub component_uuid: Uuid,
}

#[Object]
impl FileComponent {
    async fn file_uuid(&self) -> ID {
        self.file_uuid.into()
    }
    async fn component_uuid(&self) -> ID {
        self.component_uuid.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_component"]
pub struct InsertableFileComponent {
    pub file_uuid: Uuid,
    pub component_uuid: Uuid,
}

impl From<FileComponent> for InsertableFileComponent {
    fn from(ipt_data: FileComponent) -> Self {
        let FileComponent {
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
