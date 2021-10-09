use crate::errors::ServiceResult;
use crate::jwt::model::{Claims, Token};
use crate::models::user::access::token::model::UserToken;
use async_graphql::Context;
use diesel::prelude::PgConnection;
use uuid::Uuid;

/// Show tokens for user
pub(crate) fn show_user_tokens(
    logged_user_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<Vec<UserToken>> {
    use crate::models::user::access::token::show_tokens;

    show_tokens(
        logged_user_uuid,
        conn, 
    )
}

/// Get token without removing the old one
pub(crate) fn get_user_token(
    cxt: &Context<'_>,
) -> ServiceResult<Token> {
    use crate::models::user::access::token::update;

    update(cxt, false)
}

/// Update token with removing the old one
pub(crate) fn update_user_token(
    cxt: &Context<'_>,
) -> ServiceResult<Token> {
    use crate::models::user::access::token::update;

    update(cxt, true)
}

/// Decode token and return get of token data
pub(crate) fn decode_user_token(
    cxt: &Context<'_>,
) -> ServiceResult<Claims> {
    use crate::models::user::access::token::token_from_cxt;
    use crate::models::user::access::token::decode;

    decode(
        &token_from_cxt(cxt)?
    )
}

/// Delete target token for logged user
pub(crate) fn delete_target_token(
    logged_user_uuid: &Uuid,
    target_token: &str,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::models::user::access::token::delete_user_token;

    delete_user_token(
        logged_user_uuid,
        target_token,
        conn,
    )
}

/// Delete all tokens for logged user
pub(crate) fn delete_tokens(
    logged_user_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<i32> {
    use crate::models::user::access::token::delete_all_tokens;

    delete_all_tokens(
        logged_user_uuid,
        conn,
    )
}
