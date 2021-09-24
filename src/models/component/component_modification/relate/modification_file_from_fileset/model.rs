use crate::schema::*;
use crate::models::component::component_modification::fileset_for_program::model::FilesetProgram;
use crate::models::relate_ref::file::model::ShowFile;
// use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(fileset_uuid, file_uuid)]
#[belongs_to(FilesetProgram, foreign_key = "fileset_uuid")]
#[belongs_to(ShowFile, foreign_key = "file_uuid")]
#[table_name = "modification_file_from_fileset"]
pub struct ModificationFileFromFileset {
    pub fileset_uuid: Uuid,
    pub file_uuid: Uuid,
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub struct ModificationFileFromFilesetRelatedData {
    pub fileset_uuid: Uuid,
    pub files: Vec<ShowFile>,
}

impl From<(ModificationFileFromFileset, Vec<ShowFile>)> for ModificationFileFromFilesetRelatedData {
    fn from(data: (ModificationFileFromFileset, Vec<ShowFile>)) -> Self {
        Self {
            fileset_uuid: data.0.fileset_uuid,
            files: data.1,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptModificationFileFromFilesetData {
    pub fileset_uuid: Uuid,
    pub filename: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelModificationFileFromFilesetData {
    pub fileset_uuid: Uuid,
    pub file_uuids: Vec<Uuid>,
}

#[derive(Debug, Insertable)]
#[table_name = "modification_file_from_fileset"]
pub struct InsertableModificationFileFromFileset {
    pub fileset_uuid: Uuid,
    pub file_uuid: Uuid,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct ShowFileOfFileset {
    pub fileset_uuid: Uuid,
    pub show_file: ShowFile,
}
