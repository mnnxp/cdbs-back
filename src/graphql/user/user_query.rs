use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::errors::ServiceResult;
use crate::jwt::model::{Claims, Token};
use crate::models::user;
use crate::models::user::model::{ShowUserShort, SlimUser, UserAndRelatedData};
use crate::models::user::notification::model::Notification;
use crate::models::user::notification::service as notification;
use crate::models::user::service::token::model::UserToken;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    // get user info by uuid
    async fn users(
        &self,
        cxt: &Context<'_>,
        users_uuids: Vec<String>,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        // authorization check
        user::get_logged_user_uuid(cxt, true)?;

        let mut target_users_uuids = Vec::new();
        for x in users_uuids.iter() {
            target_users_uuids.push(Uuid::parse_str(x).unwrap());
        }

        user::service::list::find_users_by_uuids(cxt, &target_users_uuids)
    }

    async fn user(
        &self,
        cxt: &Context<'_>,
        user_uuid: String,
    ) -> ServiceResult<UserAndRelatedData> {
        // authorization check
        let logged_user_uuid: Uuid = user::get_logged_user_uuid(cxt, true)?;

        user::service::list::find_user_by_uuid(
            cxt,
            &Uuid::parse_str(&user_uuid)?,
            &logged_user_uuid,
        )
    }

    // return SlimUser data auth user
    async fn myself(&self, cxt: &Context<'_>) -> ServiceResult<SlimUser> {
        // authorization check
        user::util::check_authorized(cxt)?;
        // get the token of the authorized user
        let token_data = user::service::token::token_from_cxt(cxt)?;
        // decode token
        let token_data = user::service::token::decode(&token_data)?;
        // get SlimUser from jwt
        user::service::token::get_slim_user(token_data)
    }

    async fn show_tokens(&self, cxt: &Context<'_>) -> ServiceResult<Vec<UserToken>> {
        // authorization check
        let logged_user_uuid = user::get_logged_user_uuid(cxt, true)?;
        user::service::token::show_tokens(cxt, logged_user_uuid)
    }

    async fn get_token(&self, cxt: &Context<'_>) -> ServiceResult<Token> {
        user::service::token::update(cxt, false)
    }

    async fn update_token(&self, cxt: &Context<'_>) -> ServiceResult<Token> {
        user::service::token::update(cxt, true)
    }

    async fn decode_token(&self, cxt: &Context<'_>) -> ServiceResult<Claims> {
        // authorization check
        user::util::check_authorized(cxt)?;
        let token = user::service::token::token_from_cxt(cxt)?;
        user::service::token::decode(&token)
    }

    async fn delete_token(&self, cxt: &Context<'_>, token: String) -> ServiceResult<String> {
        let logged_user_uuid = user::get_logged_user_uuid(cxt, true)?;
        let deactivated_tokens = format!(
            "removed {} token.",
            // deactivate all user token
            user::service::token::delete_user_token(cxt, token.as_str(), logged_user_uuid,)?
        );
        Ok(deactivated_tokens)
    }

    async fn delete_all_tokens(&self, cxt: &Context<'_>) -> ServiceResult<String> {
        let logged_user_uuid = user::get_logged_user_uuid(cxt, true)?;
        let deactivated_tokens = format!(
            "removed {} tokens.",
            // deactivate all user token
            user::service::token::delete_all_tokens(cxt, logged_user_uuid,)?
        );
        Ok(deactivated_tokens)
    }

    async fn logout(&self, cxt: &Context<'_>) -> ServiceResult<String> {
        // removed user token
        user::service::logout(cxt)
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

        let logged_user_uuid = user::get_logged_user_uuid(cxt, true)?;

        notification::list::get_notifications(
            cxt,
            &select_ids, 
            &logged_user_uuid,
            limit,
            offset
        )
    }
}
