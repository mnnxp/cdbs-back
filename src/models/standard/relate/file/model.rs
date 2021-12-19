use crate::schema::*;
use crate::models::standard::model::Standard;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Standard
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, SimpleObject, Debug)]
#[primary_key(file_uuid, standard_uuid)]
#[belongs_to(ShowFileRelatedData, foreign_key = "file_uuid")]
#[belongs_to(Standard, foreign_key = "standard_uuid")]
#[table_name = "file_to_standard"]
pub struct StandardFile {
    pub file_uuid: Uuid,
    pub standard_uuid: Uuid,
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
    pub filenames: Vec<String>,
    pub standard_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub struct DeleteStandardFileData {
    pub file_uuid: Uuid,
    pub standard_uuid: Uuid,
}
