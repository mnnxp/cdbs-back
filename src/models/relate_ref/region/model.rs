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
#[primary_key(id_region, id_lang)]
#[belongs_to(Region, foreign_key = "id_region")]
#[belongs_to(Company, foreign_key = "id_region")]
#[belongs_to(CompanyRepresent, foreign_key = "id_region")]
#[belongs_to(User, foreign_key = "id_region")]
#[belongs_to(Language, foreign_key = "id_lang")]
#[table_name = "region_translate_list"]
pub struct RegionTranslateList {
    pub id_region: i32,
    pub id_lang: i32,
    pub region: String,
}

#[Object]
impl RegionTranslateList {
    async fn id_region(&self) -> &i32 {
        &self.id_region
    }
    async fn id_lang(&self) -> &i32 {
        &self.id_lang
    }
    async fn region(&self) -> &String {
        &self.region
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptRegionTranslateListData {
    pub id_lang: i32,
    pub region: String,
}

#[derive(Debug, Insertable)]
#[table_name = "region_translate_list"]
pub struct InsertableRegionTranslateList {
    pub id_region: i32,
    pub id_lang: i32,
    pub region: String,
}
