use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::jwt::model::{Claims, Token};
use crate::models::user::{util::check_authorized, get_logged_user_uuid};
use crate::models::user::model::{ShowUserShort, SlimUser, UserAndRelatedData};
use crate::models::user::notification::model::Notification;
use crate::models::user::notification::service as notification;
use crate::models::user::access::token::model::UserToken;
use crate::models::user::get_set_language;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    // get user info by uuid
    async fn users(
        &self,
        cxt: &Context<'_>,
        users_uuids: Vec<Uuid>,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        use crate::models::user::service::list::find_users_by_uuids;

        // authorization check
        let _logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        find_users_by_uuids(
            // &logged_user_uuid,
            &users_uuids,
            conn,
        )
    }

    async fn user(
        &self,
        cxt: &Context<'_>,
        user_uuid: Uuid,
    ) -> ServiceResult<UserAndRelatedData> {
        use crate::models::user::service::list::find_user_by_uuid;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        find_user_by_uuid(
            &logged_user_uuid,
            &user_uuid,
            &get_set_language(cxt),
            conn,
        )
    }

    // return SlimUser data auth user
    async fn myself(&self, cxt: &Context<'_>) -> ServiceResult<SlimUser> {
        use crate::models::user::access::token::token_from_cxt;
        use crate::models::user::access::token::decode;
        use crate::models::user::access::token::get_slim_user;

        // authorization check
        check_authorized(cxt)?;
        // get the token of the authorized user
        let token_data = token_from_cxt(cxt)?;
        // decode token
        let token_data = decode(&token_data)?;
        // get SlimUser from jwt
        get_slim_user(token_data)
    }

    async fn show_tokens(&self, cxt: &Context<'_>) -> ServiceResult<Vec<UserToken>> {
        use crate::models::user::access::token::show_tokens;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        show_tokens(cxt, logged_user_uuid)
    }

    async fn get_token(&self, cxt: &Context<'_>) -> ServiceResult<Token> {
        use crate::models::user::access::token::update;
        update(cxt, false)
    }

    async fn update_token(&self, cxt: &Context<'_>) -> ServiceResult<Token> {
        use crate::models::user::access::token::update;
        update(cxt, true)
    }

    async fn decode_token(&self, cxt: &Context<'_>) -> ServiceResult<Claims> {
        use crate::models::user::access::token::token_from_cxt;
        use crate::models::user::access::token::decode;

        // authorization check
        check_authorized(cxt)?;

        let token = token_from_cxt(cxt)?;

        decode(&token)
    }

    async fn delete_token(&self, cxt: &Context<'_>, token: String) -> ServiceResult<String> {
        use crate::models::user::access::token::delete_user_token;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let deactivated_tokens = format!(
            "removed {} token.",
            // deactivate all user token
            delete_user_token(
                cxt,
                token.as_str(),
                logged_user_uuid,
            )?
        );

        Ok(deactivated_tokens)
    }

    async fn delete_all_tokens(&self, cxt: &Context<'_>) -> ServiceResult<String> {
        use crate::models::user::access::token::delete_all_tokens;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let deactivated_tokens = format!(
            "removed {} tokens.",
            // deactivate all user token
            delete_all_tokens(
                cxt,
                logged_user_uuid,
            )?
        );

        Ok(deactivated_tokens)
    }

    async fn logout(&self, cxt: &Context<'_>) -> ServiceResult<String> {
        use crate::models::user::service::logout;
        // removed user token
        logout(cxt)
    }

    async fn notifications(
        &self,
        cxt: &Context<'_>,
        select_ids: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Notification>> {
        let select_ids: Vec<i32>  = select_ids.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        notification::list::get_notifications(
            cxt,
            &select_ids,
            &logged_user_uuid,
            limit,
            offset
        )
    }
}
