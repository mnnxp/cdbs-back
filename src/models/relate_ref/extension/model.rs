use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = extension_ref)]
pub(crate) struct Extension {
    pub(crate) id: i32,
    pub(crate) extension: String,
    pub(crate) program_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = extension_ref)]
pub(crate) struct InsertableExtension {
    pub(crate) extension: String,
    pub(crate) program_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptExtensionData {
    pub(crate) extension: String,
    pub(crate) program_id: i32,
}

impl From<&IptExtensionData> for InsertableExtension {
    fn from(data: &IptExtensionData) -> Self {
        Self {
            extension: data.extension.clone(),
            program_id: data.program_id,
        }
    }
}
