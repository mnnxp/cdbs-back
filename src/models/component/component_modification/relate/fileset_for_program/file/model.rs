use crate::schema::*;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = modification_file_from_fileset)]
pub(crate) struct ModificationFileFromFileset {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuid: Uuid,
}

/// Data for requesting new files to be added to the file set
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptModificationFileFromFilesetData {
    /// UUID of the file set to which the new files will be added
    pub(crate) fileset_uuid: Uuid,
    /// Names of files to be added to the file set
    pub(crate) filenames: Vec<String>,
    /// Change comment has length limit of 225.
    /// Exceeding the limit will be replaced with `...`.
    #[graphql(default = "")]
    pub(crate) commit_msg: String,
}

/// Data for a request to delete files from a set of files
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelModificationFileFromFilesetData {
    /// UUID of the file set to which the files to be deleted belong
    pub(crate) fileset_uuid: Uuid,
    /// UUIDs of files to be deleted from the file set
    pub(crate) file_uuids: Vec<Uuid>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = modification_file_from_fileset)]
pub(crate) struct InsertableModificationFileFromFileset {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuid: Uuid,
}

/// Data for requesting file data from a set of files
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptFileOfFilesetArg {
    /// UUID of file set
    pub(crate) fileset_uuid: Uuid,
    /// UUIDs of files to filter (optional)
    pub(crate) file_uuids: Option<Vec<Uuid>>,
}

#[derive(Debug)]
pub(crate) struct FileOfFilesetArg {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
}

impl From<IptFileOfFilesetArg> for FileOfFilesetArg {
    fn from(data: IptFileOfFilesetArg) -> Self {
        let IptFileOfFilesetArg {
            fileset_uuid,
            file_uuids,
        } = data;

        Self {
            fileset_uuid,
            file_uuids: file_uuids.unwrap_or_default(),
        }
    }
}
