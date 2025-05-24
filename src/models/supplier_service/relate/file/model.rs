use crate::schema::*;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Service
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = file_to_service)]
pub(crate) struct ServiceFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) service_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = file_to_service)]
pub(crate) struct InsertableServiceFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) service_uuid: Uuid,
}

impl From<ServiceFile> for InsertableServiceFile {
    fn from(ipt_data: ServiceFile) -> Self {
        let ServiceFile {
            file_uuid,
            service_uuid,
            ..
        } = ipt_data;

        Self {
            file_uuid,
            service_uuid,
        }
    }
}

/// Data for request to add service files (illustrations, documentation, etc.)
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptServiceFilesData {
    /// Names of files to be uploaded (list)
    pub(crate) filenames: Vec<String>,
    /// UUID of the service
    pub(crate) service_uuid: Uuid,
    /// Change comment has length limit of 225.
    /// Exceeding the limit will be replaced with `...`.
    #[graphql(default = "")]
    pub(crate) commit_msg: String,
}

/// Data for requesting deletion files of service
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct DeleteServiceFileData {
    /// UUID of the file to be deleted
    pub(crate) file_uuid: Uuid,
    /// UUID of service
    pub(crate) service_uuid: Uuid,
}
