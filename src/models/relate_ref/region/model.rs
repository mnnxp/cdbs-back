use crate::schema::*;
use async_graphql::*;

// Region models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = region_ref)]
pub(crate) struct Region {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = region_ref)]
pub(crate) struct InsertableRegion {
    pub(crate) id: i32,
}

// Region translations
/// Global (conditional) region data with localization
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = region_translate_list)]
pub(crate) struct RegionTranslateList {
    /// Region identifier
    pub(crate) region_id: i32,
    /// Name localization language identifier
    pub(crate) lang_id: i32,
    /// Name of global (conditional) region
    pub(crate) region: String,
}

/// Data for request to add a global (conditional) region
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptRegionTranslateListData {
    /// Name localization language identifier
    pub(crate) lang_id: i32,
    /// Name of global (conditional) region
    pub(crate) region: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = region_translate_list)]
pub(crate) struct InsertableRegionTranslateList {
    pub(crate) region_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) region: String,
}