use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

/// File linkage and component modification data
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[diesel(primary_key(file_uuid, modification_uuid))]
#[diesel(belongs_to(ShowFileRelatedData, foreign_key = file_uuid))]
#[diesel(belongs_to(ComponentModification, foreign_key = modification_uuid))]
#[diesel(table_name = file_to_modification)]
pub(crate) struct FileModification {
    /// UUID of linked file
    pub(crate) file_uuid: Uuid,
    /// UUID of component modification
    pub(crate) modification_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = file_to_modification)]
pub(crate) struct InsertableFileModification {
    pub(crate) file_uuid: Uuid,
    pub(crate) modification_uuid: Uuid,
}

impl From<FileModification> for InsertableFileModification {
    fn from(ipt_data: FileModification) -> Self {
        let FileModification {
            file_uuid,
            modification_uuid,
            ..
        } = ipt_data;

        Self {
            file_uuid,
            modification_uuid,
        }
    }
}

/// Data of the request to add files to the component modification
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptModificationFilesData {
    /// Names of files to be added
    pub(crate) filenames: Vec<String>,
    /// UUID of the component modification
    pub(crate) modification_uuid: Uuid,
    /// Change comment has length limit of 225.
    /// Exceeding the limit will be replaced with `...`.
    #[graphql(default = "")]
    pub(crate) commit_msg: String,
}

/// Component modification file deletion request data
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct DelModificationFileData {
    /// UUID of the component modification file (to be deleted)
    pub(crate) file_uuid: Uuid,
    /// UUID of the component modification
    pub(crate) modification_uuid: Uuid,
}
