use async_graphql::InputObject;
use uuid::Uuid;

/// Data for changing the user-owner of the service
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeOwnerService {
    /// UUID of the service
    pub(crate) service_uuid: Uuid,
    /// UUID of the new service owner user
    pub(crate) new_owner_user_uuid: Uuid,
}

/// Data to change the default access type of the service
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeTypeAccessService {
    /// Service UUID
    pub(crate) service_uuid: Uuid,
    /// Access type (level) identifier
    pub(crate) new_type_access_id: i32,
}
