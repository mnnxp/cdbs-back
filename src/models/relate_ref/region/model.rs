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
    pub(crate) region_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) region: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptRegionTranslateListData {
    pub(crate) lang_id: i32,
    pub(crate) region: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = region_translate_list)]
pub(crate) struct InsertableRegionTranslateList {
    pub(crate) region_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) region: String,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptRegionArg {
    pub(crate) region_ids:  Option<Vec<i32>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct RegionArg {
    pub(crate) region_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for RegionArg {
    fn default() -> Self {
        Self {
            region_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptRegionArg> for RegionArg {
    fn from(data: IptRegionArg) -> Self {
        let IptRegionArg {
            region_ids,
            limit,
            offset,
        } = data;

        Self {
            region_ids: region_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
