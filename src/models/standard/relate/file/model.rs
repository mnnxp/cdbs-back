use crate::schema::*;
use crate::models::standard::model::Standard;
use crate::models::relate_ref::file::model::ShowFileForDownload;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Standard
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(file_uuid, standard_uuid)]
#[belongs_to(ShowFileForDownload, foreign_key = "file_uuid")]
#[belongs_to(Standard, foreign_key = "standard_uuid")]
#[table_name = "file_to_standard"]
pub struct StandardFile {
    pub file_uuid: Uuid,
    pub standard_uuid: Uuid,
}

#[Object]
impl StandardFile {
    async fn file_uuid(&self) -> ID {
        self.file_uuid.into()
    }
    async fn standard_uuid(&self) -> ID {
        self.standard_uuid.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_standard"]
pub struct InsertableStandardFile {
    pub file_uuid: Uuid,
    pub standard_uuid: Uuid,
}

impl From<StandardFile> for InsertableStandardFile {
    fn from(ipt_data: StandardFile) -> Self {
        let StandardFile {
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


#[derive(InputObject, Deserialize, Debug)]
pub struct IptStandardFilesData {
    pub filename: Vec<String>,
    pub standard_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub struct DeleteStandardFileData {
    pub file_uuid: Uuid,
    pub standard_uuid: Uuid,
}
