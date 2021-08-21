use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Clone, Debug)]
#[primary_key(id)]
#[table_name = "program_ref"]
pub struct Program {
    pub id: i32,
    pub name: String,
}

#[Object]
impl Program {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Insertable)]
#[table_name = "program_ref"]
pub struct InsertableProgram {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptProgramData {
    pub name: String,
}

impl From<IptProgramData> for InsertableProgram {
    fn from(data: IptProgramData) -> Self {
        let IptProgramData {
            name,
            ..
        } = data;

        Self {
            name,
        }
    }
}
