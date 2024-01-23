use async_graphql::InputObject;
use uuid::Uuid;

/// Данные для изменения пользователя-владельца стандарта
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeOwnerStandard {
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
    /// UUID нового пользователя-владельца стандарта
    pub(crate) new_owner_user_uuid: Uuid,
}

/// Данные для изменения типа доступа к стандарту по умолчанию
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct ChangeTypeAccessStandard {
    /// UUID стандарта
    pub(crate) standard_uuid: Uuid,
    /// Идентификатор типа доступа
    pub(crate) new_type_access_id: i32,
}
