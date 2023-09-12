use crate::errors::ServiceResult;
use crate::jwt::model::{Claims, Token};
use crate::models::user::access::model::UserToken;
use async_graphql::Context;
use diesel::prelude::PgConnection;
use uuid::Uuid;

/// Возвращает активные токены авторизованного пользователя.
pub(crate) fn show_user_tokens(
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UserToken>> {
    use crate::models::user::access::token::show_tokens;

    show_tokens(
        logged_user_uuid,
        conn,
    )
}

/// Генерирует токен для пользователя без удаления других действующих токенов.
/// Возвращает новый токен авторизации пользователя.
pub(crate) fn get_user_token(
    cxt: &Context<'_>,
) -> ServiceResult<Token> {
    use crate::models::user::access::token::update;

    update(cxt, false)
}

/// Генерирует токен для пользователя с деактивацией других токенов пользователя.
/// Возвращает новый токен авторизации пользователя.
pub(crate) fn update_user_token(
    cxt: &Context<'_>,
) -> ServiceResult<Token> {
    use crate::models::user::access::token::update;

    update(cxt, true)
}

/// Возвращает провайдера токена, UUID и имя пользователя пользователя, идентификатор программы пользователя,
/// дату выдачи токена и дату истечения срока действия токена.
pub(crate) fn decode_user_token(
    cxt: &Context<'_>,
) -> ServiceResult<Claims> {
    use crate::models::user::access::token::token_from_cxt;
    use crate::models::user::access::token::decode;

    decode(
        &token_from_cxt(cxt)?
    )
}

/// Деактивирует указанный токен пользователя.
pub(crate) fn delete_target_token(
    logged_user_uuid: &Uuid,
    target_token: &str,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::models::user::access::token::delete_user_token;

    delete_user_token(
        logged_user_uuid,
        target_token,
        conn,
    )
}

/// Деактивирует все токены пользователя.
pub(crate) fn delete_tokens(
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    use crate::models::user::access::token::delete_all_tokens;

    delete_all_tokens(
        logged_user_uuid,
        conn,
    )
}
