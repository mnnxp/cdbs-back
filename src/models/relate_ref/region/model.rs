use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::company::company_represent::model::CompanyRepresent;
use crate::models::user::model::User;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

// Region models
#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "region_ref"]
pub struct Region {
    pub id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "region_ref"]
pub struct InsertableRegion {
    pub id: i32,
}

// Region translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[primary_key(region_id, lang_id)]
#[belongs_to(Region, foreign_key = "region_id")]
#[belongs_to(Company, foreign_key = "region_id")]
#[belongs_to(CompanyRepresent, foreign_key = "region_id")]
#[belongs_to(User, foreign_key = "region_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "region_translate_list"]
pub struct RegionTranslateList {
    pub region_id: i32,
    pub lang_id: i32,
    pub region: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptRegionTranslateListData {
    pub lang_id: i32,
    pub region: String,
}

#[derive(Debug, Insertable)]
#[table_name = "region_translate_list"]
pub struct InsertableRegionTranslateList {
    pub region_id: i32,
    pub lang_id: i32,
    pub region: String,
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptRegionArg {
    pub region_ids:  Option<Vec<i32>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct RegionArg {
    pub region_ids: Vec<i32>,
    pub limit: i32,
    pub offset: i32,
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
