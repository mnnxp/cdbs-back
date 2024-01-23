use crate::schema::*;
use crate::models::component::param::model::ComponentParam;
use crate::models::component::component_modification::param::model::ModificationParam;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

// Param models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = param_ref)]
pub(crate) struct Param {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = param_ref)]
pub(crate) struct InsertableParam {
    pub(crate) id: i32,
}

// Param translations
/// Данные о параметре с локализацией. Параметры используются в качестве элемента характеристик
/// для добавления характеристик к компонентам, модификациям компонентов и стандартам
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[diesel(primary_key(param_id, lang_id))]
#[diesel(belongs_to(Param, foreign_key = param_id))]
#[diesel(belongs_to(ComponentParam, foreign_key = param_id))]
#[diesel(belongs_to(ModificationParam, foreign_key = param_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = param_translate_list)]
pub(crate) struct ParamTranslateList {
    /// Идентификатор параметра
    pub(crate) param_id: i32,
    /// Идентификатор языка локализации наименования
    pub(crate) lang_id: i32,
    /// Локализованное наименование параметра
    pub(crate) paramname: String,
}

/// Данные для запроса на добавление нового параметра
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptParamTranslateListData {
    /// Идентификатор языка локализации
    pub(crate) lang_id: i32,
    /// Локализованное наименование параметра
    pub(crate) paramname: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = param_translate_list)]
pub(crate) struct InsertableParamTranslateList {
    pub(crate) param_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) paramname: String,
}

/// Данные для запросов на добавление значения параметра
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptParamData {
    /// Идентификатор параметра
    pub(crate) param_id: i32,
    /// Значение параметра
    pub(crate) value: String,
}

/// Аргументы для запроса существующих на платформе параметров
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptParamArg {
    /// Фильтр по идентификаторам параметров
    pub(crate) param_ids:  Option<Vec<i32>>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ParamArg {
    pub(crate) param_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for ParamArg {
    fn default() -> Self {
        Self {
            param_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptParamArg> for ParamArg {
    fn from(data: IptParamArg) -> Self {
        let IptParamArg {
            param_ids,
            limit,
            offset,
        } = data;

        Self {
            param_ids: param_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
