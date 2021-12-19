use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Debug)]
#[primary_key(id)]
#[table_name = "extension_ref"]
pub struct Extension {
    pub id: i32,
    pub extension: String,
    pub program_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "extension_ref"]
pub struct InsertableExtension {
    pub extension: String,
    pub program_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptExtensionData {
    pub extension: String,
    pub program_id: i32,
}

impl From<&IptExtensionData> for InsertableExtension {
    fn from(data: &IptExtensionData) -> Self {
        Self {
            extension: data.extension.clone(),
            program_id: data.program_id,
        }
    }
}
