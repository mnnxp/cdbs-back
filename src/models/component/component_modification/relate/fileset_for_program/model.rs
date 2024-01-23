use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::program::model::Program;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(PartialEq, Clone, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(belongs_to(ComponentModification, foreign_key = modification_uuid))]
#[diesel(belongs_to(Program, foreign_key = program_id))]
#[diesel(table_name = fileset_for_program)]
pub(crate) struct FilesetProgram {
    pub(crate) uuid: Uuid,
    pub(crate) modification_uuid: Uuid,
    pub(crate) program_id: i32,
}

/// Данные набора файлов с указанием целевого ПО для этого набора
#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub(crate) struct FilesetProgramRelatedData {
    /// UUID набора файлов
    pub(crate) uuid: Uuid,
    /// UUID модификации компонента
    pub(crate) modification_uuid: Uuid,
    /// Данные о целевом ПО набора файлов
    pub(crate) program: Program,
}

/// Данные запроса на добавление набора файлов для модификации компонента
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptFilesetProgramData {
    /// UUID модификации компонента
    pub(crate) modification_uuid: Uuid,
    /// UUID набора файлов
    pub(crate) program_id: i32,
}

/// Данные запроса на удаление набора файлов из модификации компонента
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelFilesetProgramData {
    /// UUID модификации компонента
    pub(crate) modification_uuid: Uuid,
    /// UUID набора файлов
    pub(crate) fileset_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = fileset_for_program)]
pub(crate) struct InsertableFilesetProgram {
    pub(crate) uuid: Uuid,
    pub(crate) modification_uuid: Uuid,
    pub(crate) program_id: i32,
}

impl From<&IptFilesetProgramData> for InsertableFilesetProgram {
    fn from(ipt_data: &IptFilesetProgramData) -> Self {
        let IptFilesetProgramData {
            modification_uuid,
            program_id,
        } = ipt_data;

        Self {
            uuid: Uuid::new_v4(),
            modification_uuid: *modification_uuid,
            program_id: *program_id,
        }
    }
}

/// Данные запроса файлов из набора файлов модификации компонента
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptFilesetProgramArg {
    /// UUID модификации компонента
    pub(crate) modification_uuid: Uuid,
    /// Фильтрация по идентификаторам ПО (перечень)
    pub(crate) program_ids: Option<Vec<i32>>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct FilesetProgramArg {
    pub(crate) modification_uuid: Uuid,
    pub(crate) program_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl From<IptFilesetProgramArg> for FilesetProgramArg {
    fn from(data: IptFilesetProgramArg) -> Self {
        let IptFilesetProgramArg {
            modification_uuid,
            program_ids,
            limit,
            offset,
        } = data;

        Self {
            modification_uuid,
            program_ids: program_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
