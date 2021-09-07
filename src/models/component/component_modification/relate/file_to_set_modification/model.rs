use crate::schema::*;
use crate::models::component::component_modification::set_of_files_program::model::SetOfFilesProgram;
use crate::models::relate_ref::file::model::ShowFile;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(id_set, uuid_file)]
#[belongs_to(SetOfFilesProgram, foreign_key = "id_set")]
#[belongs_to(ShowFile, foreign_key = "uuid_file")]
#[table_name = "file_to_set_modification"]
pub struct FileToSetModification {
    pub id_set: i32,
    pub uuid_file: Uuid,
}

#[Object]
impl FileToSetModification {
    async fn id_set(&self) -> &i32 {
        &self.id_set
    }
    async fn uuid_file(&self) -> ID {
        self.uuid_file.into()
    }
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub struct FileToSetModificationRelatedData {
    pub id_set: i32,
    pub files: Vec<ShowFile>,
}

impl From<(FileToSetModification, Vec<ShowFile>)> for FileToSetModificationRelatedData {
    fn from(data: (FileToSetModification, Vec<ShowFile>)) -> Self {
        Self {
            id_set: data.0.id_set,
            files: data.1,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptFileToSetModificationData {
    pub id_set: i32,
    pub uuid_file: ID,
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_set_modification"]
pub struct InsertableFileToSetModification {
    pub id_set: i32,
    pub uuid_file: Uuid,
}

impl From<IptFileToSetModificationData> for InsertableFileToSetModification {
    fn from(ipt_data: IptFileToSetModificationData) -> Self {
        let IptFileToSetModificationData {
            id_set,
            uuid_file,
        } = ipt_data;

        Self {
            id_set,
            uuid_file: Uuid::parse_str(&uuid_file.to_string()).unwrap(),
        }
    }
}
