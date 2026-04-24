use crate::errors::ServiceResult;
use crate::auth::jwt::model::{Claims, Token};
use crate::auth::token::{show_tokens, update, decode, token_from_cxt, delete_user_token, delete_all_tokens};
use crate::auth::token::UserToken;
use async_graphql::Context;
use diesel::prelude::PgConnection;
use uuid::Uuid;

/// Возвращает активные токены авторизованного пользователя.
pub(crate) fn show_user_tokens(
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UserToken>> {
    show_tokens(logged_user_uuid, conn)
}

/// Генерирует токен для пользователя без удаления других действующих токенов.
/// Возвращает новый токен авторизации пользователя.
pub(crate) fn get_user_token(cxt: &Context<'_>) -> ServiceResult<Token> {
    update(cxt, false)
}

/// Генерирует токен для пользователя с деактивацией других токенов пользователя.
/// Возвращает новый токен авторизации пользователя.
pub(crate) fn update_user_token(cxt: &Context<'_>) -> ServiceResult<Token> {
    update(cxt, true)
}

/// Возвращает провайдера токена, UUID и имя пользователя пользователя, идентификатор программы пользователя,
/// дату выдачи токена и дату истечения срока действия токена.
pub(crate) fn decode_user_token(cxt: &Context<'_>) -> ServiceResult<Claims> {
    decode(&token_from_cxt(cxt)?)
}

/// Деактивирует указанный токен пользователя.
pub(crate) fn delete_target_token(
    logged_user_uuid: &Uuid,
    target_token: &str,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    delete_user_token(logged_user_uuid, target_token, conn)
}

/// Деактивирует все токены пользователя.
pub(crate) fn delete_tokens(
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    delete_all_tokens(logged_user_uuid, conn)
}
