use async_graphql::InputObject;
use uuid::Uuid;

// #[derive(Debug, Deserialize, InputObject)]
// pub struct ChangeOwnerCompany {
//     pub company_uuid: Uuid,
//     pub new_owner_user_uuid: Uuid,
// }

#[derive(Debug, Deserialize, InputObject)]
pub struct ChangeTypeAccessCompany {
    pub company_uuid: Uuid,
    pub new_type_access_id: i32,
}
