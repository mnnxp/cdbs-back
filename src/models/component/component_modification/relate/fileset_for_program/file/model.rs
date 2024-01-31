use crate::schema::*;
use crate::models::component::component_modification::fileset_for_program::model::FilesetProgram;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[diesel(primary_key(fileset_uuid, file_uuid))]
#[diesel(belongs_to(FilesetProgram, foreign_key = fileset_uuid))]
#[diesel(belongs_to(ShowFileRelatedData, foreign_key = file_uuid))]
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
    /// Restriction of data sampling (maximum number of records)
    pub(crate) limit: Option<i32>,
    /// Number of skipping records at the beginning (offset)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct FileOfFilesetArg {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl From<IptFileOfFilesetArg> for FileOfFilesetArg {
    fn from(data: IptFileOfFilesetArg) -> Self {
        let IptFileOfFilesetArg {
            fileset_uuid,
            file_uuids,
            limit,
            offset,
        } = data;

        Self {
            fileset_uuid,
            file_uuids: file_uuids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
