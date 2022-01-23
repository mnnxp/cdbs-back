use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::{InputObject, SimpleObject};
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[primary_key(standard_uuid, user_uuid)]
#[table_name = "user_access_to_standard"]
pub(crate) struct UserAccessStandard {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct UserAccessStandardAndRelatedData {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access: TypeAccessTranslateList,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "user_access_to_standard"]
pub(crate) struct InsertableUserAccessStandard {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptUserAccessStandardData {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
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
pub(crate) struct DelUserAccessStandardData {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
}
