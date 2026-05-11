use crate::auth::permission::PermissionTranslateList;
use crate::schema::*;
use async_graphql::InputObject;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(component_uuid, user_uuid))]
#[diesel(table_name = user_access_to_component)]
pub(crate) struct UserAccessComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// User access data to the component (part) with additional information
#[derive(Debug, Deserialize)]
pub(crate) struct UserAccessComponentAndRelatedData {
    /// UUID of the component
    pub(crate) component_uuid: Uuid,
    /// UUID of the user
    pub(crate) user_uuid: Uuid,
    /// Access level with localization
    pub(crate) permission: PermissionTranslateList,
    /// Access activity flag
    pub(crate) is_enabled: bool,
    /// Date of first access assignment
    pub(crate) created_at: NaiveDateTime,
    /// Date of access modification
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_access_to_component)]
pub(crate) struct InsertableUserAccessComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Data for requesting to create or change user access to the component
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptUserAccessComponentData {
    /// Identifier (UUID) of the component (part)
    pub(crate) component_uuid: Uuid,
    /// Identifier (UUID) of the user
    pub(crate) user_uuid: Uuid,
    /// Identifier of the type (level) of access
    pub(crate) type_access_id: i32,
}

impl From<&IptUserAccessComponentData> for InsertableUserAccessComponent {
    fn from(data_component: &IptUserAccessComponentData) -> Self {
        let IptUserAccessComponentData {
            component_uuid,
            user_uuid,
            type_access_id,
            ..
        } = data_component;

        Self {
            component_uuid: *component_uuid,
            user_uuid: *user_uuid,
            type_access_id: *type_access_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// Data for requesting deletion (deactivation) of user access to the component (part)
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct DelUserAccessComponentData {
    /// Identifier (UUID) of the component (part)
    pub(crate) component_uuid: Uuid,
    /// Identifier (UUID) of the user
    pub(crate) user_uuid: Uuid,
}
