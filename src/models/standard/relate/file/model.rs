use crate::schema::*;
use crate::models::standard::model::Standard;
use crate::models::relate_ref::file::model::ShowFile;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Standard
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(file_uuid, standard_uuid)]
#[belongs_to(ShowFile, foreign_key = "file_uuid")]
#[belongs_to(Standard, foreign_key = "standard_uuid")]
#[table_name = "file_to_standard"]
pub struct FileStandard {
    pub file_uuid: Uuid,
    pub standard_uuid: Uuid,
}

#[Object]
impl FileStandard {
    async fn file_uuid(&self) -> ID {
        self.file_uuid.into()
    }
    async fn standard_uuid(&self) -> ID {
        self.standard_uuid.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_standard"]
pub struct InsertableFileStandard {
    pub file_uuid: Uuid,
    pub standard_uuid: Uuid,
}

impl From<FileStandard> for InsertableFileStandard {
    fn from(ipt_data: FileStandard) -> Self {
        let FileStandard {
            file_uuid,
            standard_uuid,
            ..
        } = ipt_data;

        Self {
            file_uuid,
            standard_uuid,
        }
    }
}
