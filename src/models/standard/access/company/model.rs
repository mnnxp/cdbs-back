use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::{InputObject, SimpleObject};
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[primary_key(standard_uuid, company_uuid)]
#[table_name = "company_access_to_standard"]
pub struct CompanyAccessStandard {
    pub standard_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access_id: i32,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct CompanyAccessStandardAndRelatedData {
    pub standard_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access: TypeAccessTranslateList,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "company_access_to_standard"]
pub struct InsertableCompanyAccessStandard {
    pub standard_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access_id: i32,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, InputObject)]
pub struct IptCompanyAccessStandardData {
    pub standard_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access_id: i32,
}

impl From<&IptCompanyAccessStandardData> for InsertableCompanyAccessStandard {
    fn from(data_standard: &IptCompanyAccessStandardData) -> Self {
        let IptCompanyAccessStandardData {
            standard_uuid,
            company_uuid,
            type_access_id,
            ..
        } = data_standard;

        Self {
            standard_uuid: *standard_uuid,
            company_uuid: *company_uuid,
            type_access_id: *type_access_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}


#[derive(Debug, Deserialize, InputObject)]
pub struct DelCompanyAccessStandardData {
    pub standard_uuid: Uuid,
    pub company_uuid: Uuid,
}
