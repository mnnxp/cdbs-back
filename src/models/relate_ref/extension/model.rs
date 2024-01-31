use crate::schema::*;
use async_graphql::*;

/// Data about the file extension and the software associated with this extension
#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = extension_ref)]
pub(crate) struct Extension {
    /// File extension identifier
    pub(crate) id: i32,
    /// File extension
    pub(crate) extension: String,
    /// Associated software identifier
    pub(crate) program_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = extension_ref)]
pub(crate) struct InsertableExtension {
    pub(crate) extension: String,
    pub(crate) program_id: i32,
}

/// Data for a request to add a software association with an extension
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptExtensionData {
    /// File extension (e.g ".FCStd")
    pub(crate) extension: String,
    /// Identifier of the associated software
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
