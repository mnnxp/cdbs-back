use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::program::model::Program;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(PartialEq, Clone, Debug)]
#[primary_key(uuid)]
#[belongs_to(ComponentModification, foreign_key = "modification_uuid")]
#[belongs_to(Program, foreign_key = "program_id")]
#[table_name = "fileset_for_program"]
pub(crate) struct FilesetProgram {
    pub(crate) uuid: Uuid,
    pub(crate) modification_uuid: Uuid,
    pub(crate) program_id: i32,
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub(crate) struct FilesetProgramRelatedData {
    pub(crate) uuid: Uuid,
    pub(crate) modification_uuid: Uuid,
    pub(crate) program: Program,
}

// impl FilesetProgramRelatedData {
//     /// Create struct with FilesetProgram data, Program data set default
//     pub(crate) fn new(data: &FilesetProgram) -> Self {
//         Self{
//             uuid: data.uuid,
//             modification_uuid: data.modification_uuid,
//             program: Default::default(),
//         }
//     }
//
//     /// Change program data
//     pub(crate) fn put_program(&mut self, program: &Program) {
//         self.program = program.clone();
//     }
// }

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptFilesetProgramData {
    pub(crate) modification_uuid: Uuid,
    pub(crate) program_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelFilesetProgramData {
    pub(crate) modification_uuid: Uuid,
    pub(crate) fileset_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "fileset_for_program"]
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptFilesetProgramArg {
    pub(crate) modification_uuid: Uuid,
    pub(crate) program_ids: Option<Vec<i32>>,
    pub(crate) limit: Option<i32>,
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
