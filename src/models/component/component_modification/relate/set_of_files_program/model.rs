use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::program::model::Program;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(id)]
#[belongs_to(ComponentModification, foreign_key = "uuid_modification")]
#[belongs_to(Program, foreign_key = "id_program")]
#[table_name = "set_files_for_program"]
pub struct SetOfFilesProgram {
    pub id: i32,
    pub uuid_modification: Uuid,
    pub id_program: i32,
}

#[Object]
impl SetOfFilesProgram {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn uuid_modification(&self) -> ID {
        self.uuid_modification.into()
    }
    async fn id_program(&self) -> &i32 {
        &self.id_program
    }
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub struct SetOfFilesProgramRelatedData {
    pub id: i32,
    pub uuid_modification: Uuid,
    pub program: Program,
}

impl From<(SetOfFilesProgram, Program)> for SetOfFilesProgramRelatedData {
    fn from(data: (SetOfFilesProgram, Program)) -> Self {
        Self {
            id: data.0.id,
            uuid_modification: data.0.uuid_modification,
            program: data.1,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSetOfFilesProgramData {
    pub uuid_modification: ID,
    pub id_program: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "set_files_for_program"]
pub struct InsertableSetOfFilesProgram {
    pub uuid_modification: Uuid,
    pub id_program: i32,
}

impl From<IptSetOfFilesProgramData> for InsertableSetOfFilesProgram {
    fn from(ipt_data: IptSetOfFilesProgramData) -> Self {
        let IptSetOfFilesProgramData {
            uuid_modification,
            id_program,
        } = ipt_data;

        Self {
            uuid_modification: Uuid::parse_str(&uuid_modification.to_string()).unwrap(),
            id_program,
        }
    }
}
