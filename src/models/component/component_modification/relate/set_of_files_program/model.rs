use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::program::model::Program;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(id)]
#[belongs_to(ComponentModification, foreign_key = "modification_uuid")]
#[belongs_to(Program, foreign_key = "program_id")]
#[table_name = "set_files_for_program"]
pub struct SetOfFilesProgram {
    pub id: i32,
    pub modification_uuid: Uuid,
    pub program_id: i32,
}

#[Object]
impl SetOfFilesProgram {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn modification_uuid(&self) -> ID {
        self.modification_uuid.into()
    }
    async fn program_id(&self) -> &i32 {
        &self.program_id
    }
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub struct SetOfFilesProgramRelatedData {
    pub id: i32,
    pub modification_uuid: Uuid,
    pub program: Program,
}

impl From<(SetOfFilesProgram, Program)> for SetOfFilesProgramRelatedData {
    fn from(data: (SetOfFilesProgram, Program)) -> Self {
        Self {
            id: data.0.id,
            modification_uuid: data.0.modification_uuid,
            program: data.1,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSetOfFilesProgramData {
    pub modification_uuid: ID,
    pub program_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "set_files_for_program"]
pub struct InsertableSetOfFilesProgram {
    pub modification_uuid: Uuid,
    pub program_id: i32,
}

impl From<IptSetOfFilesProgramData> for InsertableSetOfFilesProgram {
    fn from(ipt_data: IptSetOfFilesProgramData) -> Self {
        let IptSetOfFilesProgramData {
            modification_uuid,
            program_id,
        } = ipt_data;

        Self {
            modification_uuid: Uuid::parse_str(&modification_uuid.to_string()).unwrap(),
            program_id,
        }
    }
}
