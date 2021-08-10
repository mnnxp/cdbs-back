use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::file::model::ShowFile;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Component
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(uuid_file, uuid_component)]
#[belongs_to(ShowFile, foreign_key = "uuid_file")]
#[belongs_to(Component, foreign_key = "uuid_component")]
#[table_name = "file_to_component"]
pub struct FileComponent {
    pub uuid_file: Uuid,
    pub uuid_component: Uuid,
}

#[Object]
impl FileComponent {
    async fn uuid_file(&self) -> ID {
        self.uuid_file.into()
    }
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_component"]
pub struct InsertableFileComponent {
    pub uuid_file: Uuid,
    pub uuid_component: Uuid,
}

impl From<FileComponent> for InsertableFileComponent {
    fn from(ipt_data: FileComponent) -> Self {
        let FileComponent {
            uuid_file,
            uuid_component,
            ..
        } = ipt_data;

        Self {
            uuid_file,
            uuid_component,
        }
    }
}
