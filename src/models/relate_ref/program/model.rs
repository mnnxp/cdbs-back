use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[primary_key(id)]
#[table_name = "program_ref"]
pub(crate) struct Program {
    pub(crate) id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "program_ref"]
pub(crate) struct InsertableProgram {
    name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptProgramData {
    pub(crate) name: String,
}

impl From<&IptProgramData> for InsertableProgram {
    fn from(data: &IptProgramData) -> Self {
        Self {
            name: data.name.clone(),
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptProgramArg {
    pub(crate) program_ids:  Option<Vec<i32>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ProgramArg {
    pub(crate) program_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for ProgramArg {
    fn default() -> Self {
        Self {
            program_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptProgramArg> for ProgramArg {
    fn from(data: IptProgramArg) -> Self {
        let IptProgramArg {
            program_ids,
            limit,
            offset,
        } = data;

        Self {
            program_ids: program_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
