use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::{InputObject, SimpleObject};
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[primary_key(component_uuid, company_uuid)]
#[table_name = "company_access_to_component"]
pub struct CompanyAccessComponent {
    pub component_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access_id: i32,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct CompanyAccessComponentAndRelatedData {
    pub component_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access: TypeAccessTranslateList,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "company_access_to_component"]
pub struct InsertableCompanyAccessComponent {
    pub component_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access_id: i32,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, InputObject)]
pub struct IptCompanyAccessComponentData {
    pub component_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access_id: i32,
}

impl From<&IptCompanyAccessComponentData> for InsertableCompanyAccessComponent {
    fn from(data_component: &IptCompanyAccessComponentData) -> Self {
        let IptCompanyAccessComponentData {
            component_uuid,
            company_uuid,
            type_access_id,
            ..
        } = data_component;

        Self {
            component_uuid: *component_uuid,
            company_uuid: *company_uuid,
            type_access_id: *type_access_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}


#[derive(Debug, Deserialize, InputObject)]
pub struct DelCompanyAccessComponentData {
    pub component_uuid: Uuid,
    pub company_uuid: Uuid,
}
