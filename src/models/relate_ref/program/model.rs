use crate::schema::*;
use async_graphql::*;

/// Data about the software or other solution used by users
#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Clone, Default, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = program_ref)]
pub(crate) struct Program {
    /// Software solution identifier
    pub(crate) id: i32,
    /// Name of the software solution
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = program_ref)]
pub(crate) struct InsertableProgram {
    name: String,
}

/// Data for request for registration of a new software solution
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptProgramData {
    /// Name of the software solution
    pub(crate) name: String,
}

impl From<&IptProgramData> for InsertableProgram {
    fn from(data: &IptProgramData) -> Self {
        Self {
            name: data.name.clone(),
        }
    }
}
