use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "extension_ref"]
pub struct Extension {
    pub id: i32,
    pub extension: String,
    pub id_program: i32,
}

#[Object]
impl Extension {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn extension(&self) -> &String {
        &self.extension
    }
    async fn id_program(&self) -> &i32 {
        &self.id_program
    }
}

#[derive(Debug, Insertable)]
#[table_name = "extension_ref"]
pub struct InsertableExtension {
    pub extension: String,
    pub id_program: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptExtensionData {
    pub extension: String,
    pub id_program: i32,
}

impl From<IptExtensionData> for InsertableExtension {
    fn from(data: IptExtensionData) -> Self {
        let IptExtensionData {
            extension,
            id_program,
            ..
        } = data;

        Self {
            extension,
            id_program,
        }
    }
}
