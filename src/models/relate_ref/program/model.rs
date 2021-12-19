use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[primary_key(id)]
#[table_name = "program_ref"]
pub struct Program {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "program_ref"]
pub(crate) struct InsertableProgram {
    name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptProgramData {
    pub name: String,
}

impl From<&IptProgramData> for InsertableProgram {
    fn from(data: &IptProgramData) -> Self {
        Self {
            name: data.name.clone(),
        }
    }
}
