use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::program::model::Program;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(PartialEq, Clone, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(belongs_to(ComponentModification, foreign_key = modification_uuid))]
#[diesel(belongs_to(Program, foreign_key = program_id))]
#[diesel(table_name = fileset_for_program)]
pub(crate) struct FilesetProgram {
    pub(crate) uuid: Uuid,
    pub(crate) modification_uuid: Uuid,
    pub(crate) program_id: i32,
}

/// File set data with the target software for this set
#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub(crate) struct FilesetProgramRelatedData {
    /// File set UUID
    pub(crate) uuid: Uuid,
    /// Component modification UUID
    pub(crate) modification_uuid: Uuid,
    /// File set target software data
    pub(crate) program: Program,
}

/// Data of the request to add a set of files for modification of a component
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptFilesetProgramData {
    /// UUID of the component modification
    pub(crate) modification_uuid: Uuid,
    /// Software identifier
    pub(crate) program_id: i32,
}

/// Request data for deleting a set of files from a component modification
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelFilesetProgramData {
    /// UUID of component modification
    pub(crate) modification_uuid: Uuid,
    /// UUID of file set
    pub(crate) fileset_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = fileset_for_program)]
pub(crate) struct InsertableFilesetProgram {
    pub(crate) uuid: Uuid,
    pub(crate) modification_uuid: Uuid,
    pub(crate) program_id: i32,
}

impl From<&IptFilesetProgramData> for InsertableFilesetProgram {
    fn from(ipt_data: &IptFilesetProgramData) -> Self {
        let IptFilesetProgramData {
            modification_uuid,
            program_id,
        } = ipt_data;

        Self {
            uuid: Uuid::new_v4(),
            modification_uuid: *modification_uuid,
            program_id: *program_id,
        }
    }
}

/// File request data from a set of component modification files
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptFilesetProgramArg {
    /// UUID of component modification
    pub(crate) modification_uuid: Uuid,
    /// Filtering by software identifiers (list)
    pub(crate) program_ids: Option<Vec<i32>>,
    /// Restriction of data sampling (maximum number of records)
    pub(crate) limit: Option<i32>,
    /// Number of skipping records at the beginning (offset)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct FilesetProgramArg {
    pub(crate) modification_uuid: Uuid,
    pub(crate) program_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl From<IptFilesetProgramArg> for FilesetProgramArg {
    fn from(data: IptFilesetProgramArg) -> Self {
        let IptFilesetProgramArg {
            modification_uuid,
            program_ids,
            limit,
            offset,
        } = data;

        Self {
            modification_uuid,
            program_ids: program_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
