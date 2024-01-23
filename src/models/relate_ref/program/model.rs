use crate::schema::*;
use async_graphql::*;

/// Данные о программном или ином решении, используемом пользователями
#[derive(Identifiable, Serialize, Deserialize, Queryable)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = program_ref)]
pub(crate) struct Program {
    /// Идентификатор программного решения
    pub(crate) id: i32,
    /// Наименование программного решения
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = program_ref)]
pub(crate) struct InsertableProgram {
    name: String,
}

/// Данные для запроса на регистрацию нового программного решения
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptProgramData {
    /// Наименование программного решения
    pub(crate) name: String,
}

impl From<&IptProgramData> for InsertableProgram {
    fn from(data: &IptProgramData) -> Self {
        Self {
            name: data.name.clone(),
        }
    }
}

/// Аргументы для запроса существующих программных решений на платформе
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptProgramArg {
    /// Фильтрация по идентификаторам программных решений
    pub(crate) program_ids:  Option<Vec<i32>>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ProgramArg {
    pub(crate) program_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for ProgramArg {
    fn default() -> Self {
        Self {
            program_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptProgramArg> for ProgramArg {
    fn from(data: IptProgramArg) -> Self {
        let IptProgramArg {
            program_ids,
            limit,
            offset,
        } = data;

        Self {
            program_ids: program_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
