use async_graphql::InputObject;
use uuid::Uuid;

#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeOwnerComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) new_owner_user_uuid: Uuid,
}

#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeTypeAccessComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) new_type_access_id: i32,
}
