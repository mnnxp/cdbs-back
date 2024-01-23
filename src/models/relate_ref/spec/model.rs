use crate::schema::*;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = spec_ref)]
pub(crate) struct Spec {
    pub(crate) id: i32,
    pub(crate) parent_spec_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = spec_ref)]
pub(crate) struct InsertableSpec {
    pub(crate) parent_spec_id: i32,
}

/// Данные о каталоге (элементе каталога) с локализацией
#[derive(SimpleObject, Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[diesel(primary_key(spec_id, lang_id))]
#[diesel(belongs_to(Spec, foreign_key = spec_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = spec_translate_list)]
pub(crate) struct SpecTranslateList {
    /// Идентификатор элемента каталога
    pub(crate) spec_id: i32,
    /// Идентификатор языка локализации наименования
    pub(crate) lang_id: i32,
    /// Локализованное наименование каталога
    pub(crate) spec: String,
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName, Clone, Debug)]
#[diesel(table_name = spec_translate_list)]
pub(crate) struct SpecId {
    pub(crate) spec_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = spec_translate_list)]
pub(crate) struct InsertableSpecTranslateList {
    pub(crate) spec_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) spec: String,
}

/// Данные каталога в виде пути. Разделитель и глубина пути задаются при формировании.
/// Например "Корень/Крепежные изделия/Болт", где "/" - разделитель, а глубина 3.
#[derive(Serialize, SimpleObject, Debug)]
pub(crate) struct SpecPath {
    /// Идентификатор элемента каталога
    pub(crate) spec_id: i32,
    /// Идентификатор языка локализации наименования
    pub(crate) lang_id: i32,
    /// Локализованное наименование каталога
    pub(crate) path: String,
}

/// Аргументы для запроса на получение путей каталогов
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSpecPathArg {
    /// Фильтрация по идентификаторам каталогов
    pub(crate) spec_ids: Option<Vec<i32>>,
    /// Символ для разделения уровней каталога (по-умолчанию "/")
    pub(crate) split_char: Option<char>,
    /// Глубина формирования каталога (по-умолчанию 3)
    pub(crate) depth_level: Option<i32>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct SpecPathArg {
    pub(crate) spec_ids: Vec<i32>,
    pub(crate) split_char: char,
    pub(crate) depth_level: i32,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for SpecPathArg {
    fn default() -> Self {
        Self {
            spec_ids: Vec::new(),
            split_char: '/',
            depth_level: 3,
            limit: 30,
            offset: 0,
        }
    }
}

impl From<IptSpecPathArg> for SpecPathArg {
    fn from(data: IptSpecPathArg) -> Self {
        let IptSpecPathArg {
            spec_ids,
            split_char,
            depth_level,
            limit,
            offset,
        } = data;

        Self {
            spec_ids: spec_ids.unwrap_or_default(),
            split_char: split_char.unwrap_or('/'),
            depth_level: depth_level.unwrap_or(3),
            limit: limit.unwrap_or(30),
            offset: offset.unwrap_or(0),
        }
    }
}

/// Аргументы для поиска каталогов
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSearchSpecArg {
    /// Текстовая строка для поиска (поисковая фраза)
    text: String,
    /// Символ для разделения уровней каталога (по-умолчанию "/")
    split_char: Option<char>,
    /// Глубина формирования каталога (по-умолчанию 3)
    depth_level: Option<i32>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct SearchSpecArg {
    pub(crate) text: String,
    pub(crate) split_char: char,
    pub(crate) depth_level: i32,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for SearchSpecArg {
    fn default() -> Self {
        Self {
            text: String::new(),
            split_char: '/',
            depth_level: 3,
            limit: 10,
            offset: 0,
        }
    }
}

impl From<IptSearchSpecArg> for SearchSpecArg {
    fn from(data: IptSearchSpecArg) -> Self {
        let IptSearchSpecArg {
            text,
            split_char,
            depth_level,
            limit,
            offset,
        } = data;

        Self {
            text,
            split_char: split_char.unwrap_or('/'),
            depth_level: depth_level.unwrap_or(3),
            limit: limit.unwrap_or(10),
            offset: offset.unwrap_or(0),
        }
    }
}

/// Аргументы для запроса получение информации о каталогах
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSpecArg {
    /// Фильтрация по идентификаторам каталогов
    pub(crate) spec_ids: Option<Vec<i32>>,
    /// Фильтрация по уровням каталогов
    pub(crate) specs_levels: Option<Vec<i32>>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct SpecArg {
    pub(crate) spec_ids: Vec<i32>,
    pub(crate) specs_levels: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for SpecArg {
    fn default() -> Self {
        Self {
            spec_ids: Vec::new(),
            specs_levels: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptSpecArg> for SpecArg {
    fn from(data: IptSpecArg) -> Self {
        let IptSpecArg {
            spec_ids,
            specs_levels,
            limit,
            offset,
        } = data;

        Self {
            spec_ids: spec_ids.unwrap_or_default(),
            specs_levels: specs_levels.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
