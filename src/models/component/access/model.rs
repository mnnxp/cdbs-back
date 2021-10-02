use async_graphql::InputObject;
use uuid::Uuid;

#[derive(Debug, Deserialize, InputObject)]
pub struct ChangeOwnerComponent {
    pub component_uuid: Uuid,
    pub new_owner_user_uuid: Uuid,
}

#[derive(Debug, Deserialize, InputObject)]
pub struct ChangeTypeAccessComponent {
    pub component_uuid: Uuid,
    pub new_type_access_uuid: i32,
}
