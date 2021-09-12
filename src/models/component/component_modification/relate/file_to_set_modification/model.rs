use crate::schema::*;
use crate::models::component::component_modification::set_of_files_program::model::SetOfFilesProgram;
use crate::models::relate_ref::file::model::ShowFile;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(set_id, file_uuid)]
#[belongs_to(SetOfFilesProgram, foreign_key = "set_id")]
#[belongs_to(ShowFile, foreign_key = "file_uuid")]
#[table_name = "file_to_set_modification"]
pub struct FileToSetModification {
    pub set_id: i32,
    pub file_uuid: Uuid,
}

#[Object]
impl FileToSetModification {
    async fn set_id(&self) -> &i32 {
        &self.set_id
    }
    async fn file_uuid(&self) -> ID {
        self.file_uuid.into()
    }
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub struct FileToSetModificationRelatedData {
    pub set_id: i32,
    pub files: Vec<ShowFile>,
}

impl From<(FileToSetModification, Vec<ShowFile>)> for FileToSetModificationRelatedData {
    fn from(data: (FileToSetModification, Vec<ShowFile>)) -> Self {
        Self {
            set_id: data.0.set_id,
            files: data.1,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptFileToSetModificationData {
    pub set_id: i32,
    pub file_uuid: ID,
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_set_modification"]
pub struct InsertableFileToSetModification {
    pub set_id: i32,
    pub file_uuid: Uuid,
}

impl From<IptFileToSetModificationData> for InsertableFileToSetModification {
    fn from(ipt_data: IptFileToSetModificationData) -> Self {
        let IptFileToSetModificationData {
            set_id,
            file_uuid,
        } = ipt_data;

        Self {
            set_id,
            file_uuid: Uuid::parse_str(&file_uuid.to_string()).unwrap(),
        }
    }
}
