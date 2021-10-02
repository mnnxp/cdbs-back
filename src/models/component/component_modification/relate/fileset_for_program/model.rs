use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::program::model::Program;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(uuid)]
#[belongs_to(ComponentModification, foreign_key = "modification_uuid")]
#[belongs_to(Program, foreign_key = "program_id")]
#[table_name = "fileset_for_program"]
pub struct FilesetProgram {
    pub uuid: Uuid,
    pub modification_uuid: Uuid,
    pub program_id: i32,
}

#[Object]
impl FilesetProgram {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn modification_uuid(&self) -> ID {
        self.modification_uuid.into()
    }
    async fn program_id(&self) -> &i32 {
        &self.program_id
    }
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub struct FilesetProgramRelatedData {
    pub uuid: Uuid,
    pub modification_uuid: Uuid,
    pub program: Program,
}

impl From<(FilesetProgram, Program)> for FilesetProgramRelatedData {
    fn from(data: (FilesetProgram, Program)) -> Self {
        Self {
            uuid: data.0.uuid,
            modification_uuid: data.0.modification_uuid,
            program: data.1,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptFilesetProgramData {
    pub modification_uuid: Uuid,
    pub program_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelFilesetProgramData {
    pub modification_uuid: Uuid,
    pub fileset_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "fileset_for_program"]
pub struct InsertableFilesetProgram {
    pub uuid: Uuid,
    pub modification_uuid: Uuid,
    pub program_id: i32,
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
