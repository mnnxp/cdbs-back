use async_graphql::InputObject;
use uuid::Uuid;

/// Data for a request to transfer ownership of a component (part) to another user
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeOwnerComponent {
    /// UUID of the component (part)
    pub(crate) component_uuid: Uuid,
    /// UUID of the user to set it as the new owner of the component
    pub(crate) new_owner_user_uuid: Uuid,
}

/// Data for a request to change the basic type of access to a component (part)
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeTypeAccessComponent {
    /// UUID of the component (part)
    pub(crate) component_uuid: Uuid,
    /// Identifier of the access type to be set
    pub(crate) new_type_access_id: i32,
}
