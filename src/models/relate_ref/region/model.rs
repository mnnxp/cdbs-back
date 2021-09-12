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

#[Object]
impl Region {
    async fn id(&self) -> &i32 {
        &self.id
    }
}

#[derive(Debug, Insertable)]
#[table_name = "region_ref"]
pub struct InsertableRegion {
    pub id: i32,
}

// Region translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Default, Debug)]
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

#[Object]
impl RegionTranslateList {
    async fn region_id(&self) -> &i32 {
        &self.region_id
    }
    async fn lang_id(&self) -> &i32 {
        &self.lang_id
    }
    async fn region(&self) -> &String {
        &self.region
    }
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
