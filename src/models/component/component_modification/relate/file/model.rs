use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for ComponentModification
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, SimpleObject, Debug)]
#[primary_key(file_uuid, modification_uuid)]
#[belongs_to(ShowFileRelatedData, foreign_key = "file_uuid")]
#[belongs_to(ComponentModification, foreign_key = "modification_uuid")]
#[table_name = "file_to_modification"]
pub struct FileModification {
    pub file_uuid: Uuid,
    pub modification_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_modification"]
pub struct InsertableFileModification {
    pub file_uuid: Uuid,
    pub modification_uuid: Uuid,
}

impl From<FileModification> for InsertableFileModification {
    fn from(ipt_data: FileModification) -> Self {
        let FileModification {
            file_uuid,
            modification_uuid,
            ..
        } = ipt_data;

        Self {
            file_uuid,
            modification_uuid,
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptModificationFilesData {
    pub filenames: Vec<String>,
    pub modification_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub struct DelModificationFileData {
    pub file_uuid: Uuid,
    pub modification_uuid: Uuid,
}
