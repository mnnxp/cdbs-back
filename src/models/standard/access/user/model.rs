use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::{InputObject, SimpleObject};
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(standard_uuid, user_uuid))]
#[diesel(table_name = user_access_to_standard)]
pub(crate) struct UserAccessStandard {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Data on whether the user has access to the standard
#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct UserAccessStandardAndRelatedData {
    /// UUID of the standard
    pub(crate) standard_uuid: Uuid,
    /// User UUID
    pub(crate) user_uuid: Uuid,
    /// Access type (level) with localization
    pub(crate) type_access: TypeAccessTranslateList,
    /// Access activity flag
    pub(crate) is_enabled: bool,
    /// Date access was first issued to the user
    pub(crate) created_at: NaiveDateTime,
    /// Access update date
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_access_to_standard)]
pub(crate) struct InsertableUserAccessStandard {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Data for requesting user access to the standard
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptUserAccessStandardData {
    /// UUID of the standard
    pub(crate) standard_uuid: Uuid,
    /// User UUID
    pub(crate) user_uuid: Uuid,
    /// Access type (level) identifier
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

/// Data for a request to remove (deactivation) a user's access to a standard
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct DelUserAccessStandardData {
    /// UUID of the standard
    pub(crate) standard_uuid: Uuid,
    /// user UUID
    pub(crate) user_uuid: Uuid,
}
