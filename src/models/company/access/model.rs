use async_graphql::InputObject;
use uuid::Uuid;

// #[derive(Debug, Deserialize, InputObject)]
// pub(crate) struct ChangeOwnerCompany {
//     pub(crate) company_uuid: Uuid,
//     pub(crate) new_owner_user_uuid: Uuid,
// }

/// Данные для изменения типа доступа к компании по умолчанию
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeTypeAccessCompany {
    /// UUID стандарта
    pub(crate) company_uuid: Uuid,
    /// Идентификатор типа доступа
    pub(crate) new_type_access_id: i32,
}
