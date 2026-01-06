use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = spec_ref)]
pub(crate) struct Spec {
    pub(crate) id: i32,
    pub(crate) parent_spec_id: i32,
    pub(crate) path: String,
    // pub(crate) depth: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = spec_ref)]
pub(crate) struct InsertableSpec {
    pub(crate) parent_spec_id: i32,
}

/// Catalog (catalog element) data with localization
#[derive(Serialize, Deserialize, Queryable, Clone, Debug)]
#[diesel(table_name = spec_translate_list)]
pub(crate) struct SpecTranslateList {
    /// Catalog element identifier
    pub(crate) spec_id: i32,
    /// Name localization language identifier
    pub(crate) lang_id: i32,
    /// Localized catalog name
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

/// Catalog data in the form of a path. Separator and depth of the path are set during generation.
/// For example, "Root/Fasteners/Bolt", where "/" is the separator and the depth is 3.
#[derive(Serialize, SimpleObject, Debug)]
pub(crate) struct SpecPath {
    /// Catalog element identifier
    pub(crate) spec_id: i32,
    /// Name localization language identifier
    pub(crate) lang_id: i32,
    /// Localized catalog name
    pub(crate) path: String,
}

/// Arguments for requesting catalog paths
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSpecPathArg {
    /// Filtering by directory identifiers
    pub(crate) spec_ids: Option<Vec<i32>>,
    /// Symbol for separating directory levels (default is "/")
    pub(crate) split_char: Option<char>,
    /// Depth of directory formation (default is 3)
    pub(crate) depth_level: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct SpecPathArg {
    pub(crate) spec_ids: Vec<i32>,
    pub(crate) split_char: char,
    pub(crate) depth_level: i32,
}

impl Default for SpecPathArg {
    fn default() -> Self {
        Self {
            spec_ids: Vec::new(),
            split_char: '/',
            depth_level: 3,
        }
    }
}

impl From<IptSpecPathArg> for SpecPathArg {
    fn from(data: IptSpecPathArg) -> Self {
        let IptSpecPathArg {
            spec_ids,
            split_char,
            depth_level,
        } = data;

        Self {
            spec_ids: spec_ids.unwrap_or_default(),
            split_char: split_char.unwrap_or('/'),
            depth_level: depth_level.unwrap_or(3),
        }
    }
}

/// Arguments for requesting catalog paths
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSearchSpecArg {
    /// Filtering by directory identifiers
    text: String,
    /// Symbol for separating directory levels (default is "/")
    split_char: Option<char>,
    /// Depth of directory formation (default is 3)
    depth_level: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct SearchSpecArg {
    pub(crate) text: String,
    pub(crate) split_char: char,
    pub(crate) depth_level: i32,
}

impl Default for SearchSpecArg {
    fn default() -> Self {
        Self {
            text: String::new(),
            split_char: '/',
            depth_level: 3,
        }
    }
}

impl From<IptSearchSpecArg> for SearchSpecArg {
    fn from(data: IptSearchSpecArg) -> Self {
        let IptSearchSpecArg {
            text,
            split_char,
            depth_level,
        } = data;

        Self {
            text,
            split_char: split_char.unwrap_or('/'),
            depth_level: depth_level.unwrap_or(3),
        }
    }
}

/// Arguments for requesting catalog information retrieval
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSpecArg {
    /// Filtering by catalog identifiers
    pub(crate) spec_ids: Option<Vec<i32>>,
    /// Filtering by catalog levels
    pub(crate) specs_levels: Option<Vec<i32>>,
}

#[derive(Debug, Default)]
pub(crate) struct SpecArg {
    pub(crate) spec_ids: Vec<i32>,
    pub(crate) specs_levels: Vec<i32>,
}

impl From<IptSpecArg> for SpecArg {
    fn from(data: IptSpecArg) -> Self {
        let IptSpecArg {
            spec_ids,
            specs_levels,
        } = data;

        Self {
            spec_ids: spec_ids.unwrap_or_default(),
            specs_levels: specs_levels.unwrap_or_default(),
        }
    }
}
