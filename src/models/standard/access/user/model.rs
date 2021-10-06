use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::{InputObject, SimpleObject};
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[primary_key(standard_uuid, user_uuid)]
#[table_name = "user_access_to_standard"]
pub struct UserAccessStandard {
    pub standard_uuid: Uuid,
    pub user_uuid: Uuid,
    pub type_access_id: i32,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct UserAccessStandardAndRelatedData {
    pub standard_uuid: Uuid,
    pub user_uuid: Uuid,
    pub type_access: TypeAccessTranslateList,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "user_access_to_standard"]
pub struct InsertableUserAccessStandard {
    pub standard_uuid: Uuid,
    pub user_uuid: Uuid,
    pub type_access_id: i32,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, InputObject)]
pub struct IptUserAccessStandardData {
    pub standard_uuid: Uuid,
    pub user_uuid: Uuid,
    pub type_access_id: i32,
}

impl From<&IptUserAccessStandardData> for InsertableUserAccessStandard {
    fn from(data_standard: &IptUserAccessStandardData) -> Self {
        let IptUserAccessStandardData {
            standard_uuid,
            user_uuid,
            type_access_id,
            ..
        } = data_standard;

        Self {
            standard_uuid: *standard_uuid,
            user_uuid: *user_uuid,
            type_access_id: *type_access_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}


#[derive(Debug, Deserialize, InputObject)]
pub struct DelUserAccessStandardData {
    pub standard_uuid: Uuid,
    pub user_uuid: Uuid,
}
