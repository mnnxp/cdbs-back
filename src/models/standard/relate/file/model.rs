use crate::schema::*;
use crate::models::standard::model::Standard;
use crate::graphql::file::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Standard
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[diesel(primary_key(file_uuid, standard_uuid))]
#[diesel(belongs_to(ShowFileRelatedData, foreign_key = file_uuid))]
#[diesel(belongs_to(Standard, foreign_key = standard_uuid))]
#[diesel(table_name = file_to_standard)]
pub(crate) struct StandardFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) standard_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = file_to_standard)]
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

/// Data for request to update the main image of the standard
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardFaviconData {
    /// Name of the file to be uploaded
    pub(crate) filename: String,
    /// Standard UUID
    pub(crate) standard_uuid: Uuid,
}

/// Data for request to add standard files (illustrations, documentation, etc.)
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardFilesData {
    /// Names of files to be uploaded (list)
    pub(crate) filenames: Vec<String>,
    /// UUID of the standard
    pub(crate) standard_uuid: Uuid,
    /// Change comment has length limit of 225.
    /// Exceeding the limit will be replaced with `...`.
    #[graphql(default = "")]
    pub(crate) commit_msg: String,
}

/// Data for requesting deletion files of standard
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct DeleteStandardFileData {
    /// UUID of the file to be deleted
    pub(crate) file_uuid: Uuid,
    /// UUID of standard
    pub(crate) standard_uuid: Uuid,
}
