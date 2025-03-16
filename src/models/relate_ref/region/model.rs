use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::company::company_represent::model::CompanyRepresent;
use crate::models::user::model::User;
use crate::models::relate_ref::language::model::Language;
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
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[diesel(primary_key(region_id, lang_id))]
#[diesel(belongs_to(Region, foreign_key = region_id))]
#[diesel(belongs_to(Company, foreign_key = region_id))]
#[diesel(belongs_to(CompanyRepresent, foreign_key = region_id))]
#[diesel(belongs_to(User, foreign_key = region_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
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