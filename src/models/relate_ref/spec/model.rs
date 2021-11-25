use crate::schema::*;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(SimpleObject, Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "spec_ref"]
pub struct Spec {
    pub id: i32,
    pub parent_spec_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_ref"]
pub struct InsertableSpec {
    pub parent_spec_id: i32,
}

// Spec translations
#[derive(SimpleObject, Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(spec_id, lang_id)]
#[belongs_to(Spec, foreign_key = "spec_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "spec_translate_list"]
pub struct SpecTranslateList {
    pub spec_id: i32,
    pub lang_id: i32,
    pub spec: String,
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName, Clone, Debug)]
#[table_name = "spec_translate_list"]
pub struct SpecId {
    pub spec_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSpecTranslateListData {
    pub spec_id: i32,
    pub parent_spec_id: i32,
    pub lang_id: i32,
    pub spec: String,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_translate_list"]
pub struct InsertableSpecTranslateList {
    pub spec_id: i32,
    pub lang_id: i32,
    pub spec: String,
}

impl From<IptSpecTranslateListData> for InsertableSpec {
    fn from(ipt_data: IptSpecTranslateListData) -> Self {
        let IptSpecTranslateListData {
            parent_spec_id,
            ..
        } = ipt_data;

        Self {
            parent_spec_id
        }
    }
}

#[derive(Serialize, SimpleObject, Debug)]
pub struct SpecPath {
    pub spec_id: i32,
    pub lang_id: i32,
    pub path: String,
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptSpecPathArg {
    pub spec_ids: Option<Vec<i32>>,
    pub split_char: Option<char>,
    pub depth_level: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct SpecPathArg {
    pub spec_ids: Vec<i32>,
    pub split_char: char,
    pub depth_level: i32,
    pub limit: i32,
    pub offset: i32,
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
pub struct IptSearchSpecArg {
    text: String,
    split_char: Option<char>,
    depth_level: Option<i32>,
    limit: Option<i32>,
    offset: Option<i32>,
}

#[derive(Debug)]
pub struct SearchSpecArg {
    pub text: String,
    pub split_char: char,
    pub depth_level: i32,
    pub limit: i32,
    pub offset: i32,
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
pub struct IptSpecArg {
    pub spec_ids: Option<Vec<i32>>,
    pub specs_levels: Option<Vec<i32>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct SpecArg {
    pub spec_ids: Vec<i32>,
    pub specs_levels: Vec<i32>,
    pub limit: i32,
    pub offset: i32,
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
