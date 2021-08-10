use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::file::model::ShowFile;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for ComponentModification
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(uuid_file, uuid_modification)]
#[belongs_to(ShowFile, foreign_key = "uuid_file")]
#[belongs_to(ComponentModification, foreign_key = "uuid_modification")]
#[table_name = "file_to_modification"]
pub struct FileModification {
    pub uuid_file: Uuid,
    pub uuid_modification: Uuid,
}

#[Object]
impl FileModification {
    async fn uuid_file(&self) -> ID {
        self.uuid_file.into()
    }
    async fn uuid_modification(&self) -> ID {
        self.uuid_modification.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_modification"]
pub struct InsertableFileModification {
    pub uuid_file: Uuid,
    pub uuid_modification: Uuid,
}

impl From<FileModification> for InsertableFileModification {
    fn from(ipt_data: FileModification) -> Self {
        let FileModification {
            uuid_file,
            uuid_modification,
            ..
        } = ipt_data;

        Self {
            uuid_file,
            uuid_modification,
        }
    }
}
