use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Structures for Component
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[diesel(primary_key(file_uuid, component_uuid))]
#[diesel(belongs_to(ShowFileRelatedData, foreign_key = file_uuid))]
#[diesel(belongs_to(Component, foreign_key = component_uuid))]
#[diesel(table_name = file_to_component)]
pub(crate) struct ComponentFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) component_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = file_to_component)]
pub(crate) struct InsertableComponentFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) component_uuid: Uuid,
}

impl From<ComponentFile> for InsertableComponentFile {
    fn from(ipt_data: ComponentFile) -> Self {
        let ComponentFile {
            file_uuid,
            component_uuid,
            ..
        } = ipt_data;

        Self {
            file_uuid,
            component_uuid,
        }
    }
}

/// Данные для запроса на обновение основного изображения компонента
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentFaviconData {
    /// Наименование загружаемого файла
    pub(crate) filename: String,
    /// UUID компонента
    pub(crate) component_uuid: Uuid,
}

/// Данные запроса на добавление файлов компонента
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentFilesData {
    /// Наименование загружаемых файлов (перечень)
    pub(crate) filenames: Vec<String>,
    /// UUID компонента
    pub(crate) component_uuid: Uuid,
}

/// Данные запроса на удаление файлов компонента
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct DelComponentFileData {
    /// UUID удаляемых файлов (перечень)
    pub(crate) file_uuid: Uuid,
    /// UUID компонента
    pub(crate) component_uuid: Uuid,
}
