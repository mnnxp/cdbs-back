use crate::schema::*;
use crate::models::standard::model::Standard;
use crate::models::relate_ref::file::model::ShowFile;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Standard
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(uuid_file, uuid_standard)]
#[belongs_to(ShowFile, foreign_key = "uuid_file")]
#[belongs_to(Standard, foreign_key = "uuid_standard")]
#[table_name = "file_to_standard"]
pub struct FileStandard {
    pub uuid_file: Uuid,
    pub uuid_standard: Uuid,
}

#[Object]
impl FileStandard {
    async fn uuid_file(&self) -> ID {
        self.uuid_file.into()
    }
    async fn uuid_standard(&self) -> ID {
        self.uuid_standard.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_standard"]
pub struct InsertableFileStandard {
    pub uuid_file: Uuid,
    pub uuid_standard: Uuid,
}

impl From<FileStandard> for InsertableFileStandard {
    fn from(ipt_data: FileStandard) -> Self {
        let FileStandard {
            uuid_file,
            uuid_standard,
            ..
        } = ipt_data;

        Self {
            uuid_file,
            uuid_standard,
        }
    }
}
