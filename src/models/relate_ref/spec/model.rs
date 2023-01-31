use crate::schema::*;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(SimpleObject, Identifiable, Serialize, Deserialize, Queryable, Debug)]
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

// Spec translations
#[derive(SimpleObject, Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[diesel(primary_key(spec_id, lang_id))]
#[diesel(belongs_to(Spec, foreign_key = spec_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = spec_translate_list)]
pub(crate) struct SpecTranslateList {
    pub(crate) spec_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) spec: String,
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName, Clone, Debug)]
#[diesel(table_name = spec_translate_list)]
pub(crate) struct SpecId {
    pub(crate) spec_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptSpecTranslateListData {
    pub(crate) spec_id: i32,
    pub(crate) parent_spec_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) spec: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = spec_translate_list)]
pub(crate) struct InsertableSpecTranslateList {
    pub(crate) spec_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) spec: String,
}

impl From<&IptSpecTranslateListData> for InsertableSpec {
    fn from(data: &IptSpecTranslateListData) -> Self {
        Self {
            parent_spec_id: data.parent_spec_id
        }
    }
}

#[derive(Serialize, SimpleObject, Debug)]
pub(crate) struct SpecPath {
    pub(crate) spec_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) path: String,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSpecPathArg {
    pub(crate) spec_ids: Option<Vec<i32>>,
    pub(crate) split_char: Option<char>,
    pub(crate) depth_level: Option<i32>,
    pub(crate) limit: Option<i32>,
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSearchSpecArg {
    text: String,
    split_char: Option<char>,
    depth_level: Option<i32>,
    limit: Option<i32>,
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSpecArg {
    pub(crate) spec_ids: Option<Vec<i32>>,
    pub(crate) specs_levels: Option<Vec<i32>>,
    pub(crate) limit: Option<i32>,
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
