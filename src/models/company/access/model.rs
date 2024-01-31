use async_graphql::InputObject;
use uuid::Uuid;

// #[derive(Debug, Deserialize, InputObject)]
// pub(crate) struct ChangeOwnerCompany {
//     pub(crate) company_uuid: Uuid,
//     pub(crate) new_owner_user_uuid: Uuid,
// }

/// Data for changing the default company access type
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeTypeAccessCompany {
    /// UUID of the standard
    pub(crate) company_uuid: Uuid,
    /// Access type identifier
    pub(crate) new_type_access_id: i32,
}
