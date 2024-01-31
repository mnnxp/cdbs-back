use async_graphql::InputObject;
use uuid::Uuid;

/// Data for changing the user-owner of the standard
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeOwnerStandard {
    /// UUID of the standard
    pub(crate) standard_uuid: Uuid,
    /// UUID of the new standard owner user
    pub(crate) new_owner_user_uuid: Uuid,
}

/// Data to change the default access type of the standard
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeTypeAccessStandard {
    /// Standard UUID
    pub(crate) standard_uuid: Uuid,
    /// Access type (level) identifier
    pub(crate) new_type_access_id: i32,
}
