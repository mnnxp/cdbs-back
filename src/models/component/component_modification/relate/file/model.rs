use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

/// Данные связи файла и модификации компонента
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[diesel(primary_key(file_uuid, modification_uuid))]
#[diesel(belongs_to(ShowFileRelatedData, foreign_key = file_uuid))]
#[diesel(belongs_to(ComponentModification, foreign_key = modification_uuid))]
#[diesel(table_name = file_to_modification)]
pub(crate) struct FileModification {
    /// UUID связанного файла
    pub(crate) file_uuid: Uuid,
    /// UUID модификации компонента
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

/// Данные запроса на добавление файлов к модификации компонента
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptModificationFilesData {
    /// Наименования файлов для добавления
    pub(crate) filenames: Vec<String>,
    /// UUID модификации компонента
    pub(crate) modification_uuid: Uuid,
}

/// Данные запроса на удаление файла модификации компонента
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct DelModificationFileData {
    /// UUID файла модификации компонента (который требуется удалить)
    pub(crate) file_uuid: Uuid,
    /// UUID модификации компонента
    pub(crate) modification_uuid: Uuid,
}
