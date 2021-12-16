use crate::schema::*;
use crate::models::component::component_modification::fileset_for_program::model::FilesetProgram;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
// use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(fileset_uuid, file_uuid)]
#[belongs_to(FilesetProgram, foreign_key = "fileset_uuid")]
#[belongs_to(ShowFileRelatedData, foreign_key = "file_uuid")]
#[table_name = "modification_file_from_fileset"]
pub struct ModificationFileFromFileset {
    pub fileset_uuid: Uuid,
    pub file_uuid: Uuid,
}

#[derive(Debug, SimpleObject, Clone)]
pub struct ModificationFileFromFilesetRelatedData {
    pub fileset_uuid: Uuid,
    pub files: Vec<ShowFileRelatedData>,
}

impl From<(ModificationFileFromFileset, Vec<ShowFileRelatedData>)> for ModificationFileFromFilesetRelatedData {
    fn from(data: (ModificationFileFromFileset, Vec<ShowFileRelatedData>)) -> Self {
        Self {
            fileset_uuid: data.0.fileset_uuid,
            files: data.1,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptModificationFileFromFilesetData {
    pub fileset_uuid: Uuid,
    pub filenames: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelModificationFileFromFilesetData {
    pub fileset_uuid: Uuid,
    pub file_uuids: Vec<Uuid>,
}

#[derive(Debug, Insertable)]
#[table_name = "modification_file_from_fileset"]
pub struct InsertableModificationFileFromFileset {
    pub fileset_uuid: Uuid,
    pub file_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptFileOfFilesetArg {
    pub fileset_uuid: Uuid,
    pub file_uuids: Option<Vec<Uuid>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct FileOfFilesetArg {
    pub fileset_uuid: Uuid,
    pub file_uuids: Vec<Uuid>,
    pub limit: i32,
    pub offset: i32,
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
