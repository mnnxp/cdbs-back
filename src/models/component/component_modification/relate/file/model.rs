use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for ComponentModification
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, SimpleObject, Debug)]
#[diesel(primary_key(file_uuid, modification_uuid))]
#[diesel(belongs_to(ShowFileRelatedData, foreign_key = file_uuid))]
#[diesel(belongs_to(ComponentModification, foreign_key = modification_uuid))]
#[diesel(table_name = file_to_modification)]
pub(crate) struct FileModification {
    pub(crate) file_uuid: Uuid,
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptModificationFilesData {
    pub(crate) filenames: Vec<String>,
    pub(crate) modification_uuid: Uuid,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct DelModificationFileData {
    pub(crate) file_uuid: Uuid,
    pub(crate) modification_uuid: Uuid,
}
