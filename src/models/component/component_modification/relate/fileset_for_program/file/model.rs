use crate::schema::*;
use crate::models::component::component_modification::fileset_for_program::model::FilesetProgram;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[diesel(primary_key(fileset_uuid, file_uuid))]
#[diesel(belongs_to(FilesetProgram, foreign_key = fileset_uuid))]
#[diesel(belongs_to(ShowFileRelatedData, foreign_key = file_uuid))]
#[diesel(table_name = modification_file_from_fileset)]
pub(crate) struct ModificationFileFromFileset {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuid: Uuid,
}

/// Данные для запроса на добавление новых файлов в набор файлов
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptModificationFileFromFilesetData {
    /// UUID набора файлов, к которому будут добавлены новые файлы
    pub(crate) fileset_uuid: Uuid,
    /// Наименования файлов, которые требуется добавить в набор файлов
    pub(crate) filenames: Vec<String>,
}

/// Данные для запроса на удаление файлов из набора файлов
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelModificationFileFromFilesetData {
    /// UUID набора файлов к которому относятся удаляемые файлы
    pub(crate) fileset_uuid: Uuid,
    /// UUIDs файлов, которые требуется удалить из набора
    pub(crate) file_uuids: Vec<Uuid>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = modification_file_from_fileset)]
pub(crate) struct InsertableModificationFileFromFileset {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuid: Uuid,
}

/// Данные для запроса на получение данных о файлах из набора файлов
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptFileOfFilesetArg {
    /// UUID набора файлов
    pub(crate) fileset_uuid: Uuid,
    /// UUIDs файлов для фильтрации
    pub(crate) file_uuids: Option<Vec<Uuid>>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct FileOfFilesetArg {
    pub(crate) fileset_uuid: Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl From<IptFileOfFilesetArg> for FileOfFilesetArg {
    fn from(data: IptFileOfFilesetArg) -> Self {
        let IptFileOfFilesetArg {
            fileset_uuid,
            file_uuids,
            limit,
            offset,
        } = data;

        Self {
            fileset_uuid,
            file_uuids: file_uuids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
