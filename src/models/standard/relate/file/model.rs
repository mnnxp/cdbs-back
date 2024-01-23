use crate::schema::*;
use crate::models::standard::model::Standard;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Standard
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[diesel(primary_key(file_uuid, standard_uuid))]
#[diesel(belongs_to(ShowFileRelatedData, foreign_key = file_uuid))]
#[diesel(belongs_to(Standard, foreign_key = standard_uuid))]
#[diesel(table_name = file_to_standard)]
pub(crate) struct StandardFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) standard_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = file_to_standard)]
pub(crate) struct InsertableStandardFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) standard_uuid: Uuid,
}

impl From<StandardFile> for InsertableStandardFile {
    fn from(ipt_data: StandardFile) -> Self {
        let StandardFile {
            file_uuid,
            standard_uuid,
            ..
        } = ipt_data;

        Self {
            file_uuid,
            standard_uuid,
        }
    }
}

/// Данные для запроса на обновление основного изображения стандарта
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardFaviconData {
    /// Наименование загружаемого файла
    pub(crate) filename: String,
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
}

/// Данные для запроса на добавление файлов стандарта (иллюстраций, документации и т.д.)
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardFilesData {
    /// Наименования загружаемых файлов (перечень)
    pub(crate) filenames: Vec<String>,
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
}

/// Данные для запроса на удаление файлов стандарта
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct DeleteStandardFileData {
    /// UUID файла для удаления
    pub(crate) file_uuid: Uuid,
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
}
