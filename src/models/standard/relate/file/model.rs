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
pub(crate) struct StandardFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) standard_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_standard"]
pub(crate) struct InsertableStandardFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) standard_uuid: Uuid,
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
pub(crate) struct IptStandardFaviconData {
    pub(crate) filename: String,
    pub(crate) standard_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardFilesData {
    pub(crate) filenames: Vec<String>,
    pub(crate) standard_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct DeleteStandardFileData {
    pub(crate) file_uuid: Uuid,
    pub(crate) standard_uuid: Uuid,
}
