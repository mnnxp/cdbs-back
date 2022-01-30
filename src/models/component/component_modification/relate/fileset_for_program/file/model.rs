use crate::schema::*;
use crate::models::component::component_modification::fileset_for_program::model::FilesetProgram;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(fileset_uuid, file_uuid)]
#[belongs_to(FilesetProgram, foreign_key = "fileset_uuid")]
#[belongs_to(ShowFileRelatedData, foreign_key = "file_uuid")]
#[table_name = "modification_file_from_fileset"]
pub(crate) struct ModificationFileFromFileset {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuid: Uuid,
}

#[derive(Debug, SimpleObject, Clone)]
pub(crate) struct ModificationFileFromFilesetRelatedData {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) files: Vec<ShowFileRelatedData>,
}

// impl ModificationFileFromFilesetRelatedData {
//     /// Create struct with FilesetProgram data, Program data set default
//     pub(crate) fn new(fileset_uuid: &Uuid) -> Self {
//         Self{
//             fileset_uuid: *fileset_uuid,
//             files: Vec::new(),
//         }
//     }
//
//     /// Change files data
//     pub(crate) fn put_files(&mut self, files: Vec<ShowFileRelatedData>) {
//         self.files = files;
//     }
// }

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptModificationFileFromFilesetData {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) filenames: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelModificationFileFromFilesetData {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
}

#[derive(Debug, Insertable)]
#[table_name = "modification_file_from_fileset"]
pub(crate) struct InsertableModificationFileFromFileset {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptFileOfFilesetArg {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuids: Option<Vec<Uuid>>,
    pub(crate) limit: Option<i32>,
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
