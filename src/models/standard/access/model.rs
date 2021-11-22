use async_graphql::InputObject;
use uuid::Uuid;

#[derive(Debug, Deserialize, InputObject)]
pub struct ChangeOwnerStandard {
    pub standard_uuid: Uuid,
    pub new_owner_user_uuid: Uuid,
}

#[derive(Debug, Deserialize, InputObject)]
pub struct ChangeTypeAccessStandard {
    pub standard_uuid: Uuid,
    pub new_type_access_id: i32,
}
