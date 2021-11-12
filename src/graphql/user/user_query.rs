use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::jwt::model::{Claims, Token};
use crate::models::user::access::logged::{check_authorized, get_logged_user_uuid};
use crate::models::user::model::{
    ShowUserShort, SlimUser, UserAndRelatedData, ShowUserAndRelatedData, UsersArg, IptUsersArg,
};
use crate::models::user::notification::model::ShowNotification;
use crate::models::user::access::model::UserToken;
use crate::models::relate_ref::language::get_set_language;

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
        arguments: Option<IptUsersArg>,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        use crate::models::user::service::list::get_users;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: UsersArg = match arguments {
            Some(args) => UsersArg::from(args),
            None => UsersArg::default(),
        };

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_users(
            &logged_user_uuid,
            &arguments,
            conn,
        )
    }

    async fn user(
        &self,
        cxt: &Context<'_>,
        user_uuid: Option<Uuid>,
        username: Option<String>,
    ) -> ServiceResult<ShowUserAndRelatedData> {
        use crate::models::user::service::list::get_user_data;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_user_data(
            &logged_user_uuid,
            &user_uuid,
            &username,
            &get_set_language(cxt),
            conn
        )
    }

    // return SlimUser data auth user
    async fn myself(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<SlimUser> {
        use crate::models::user::service::list::get_self_slim_data;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_self_slim_data(
            &logged_user_uuid,
            conn,
        )
    }

    // return self data user
    async fn self_data(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<UserAndRelatedData> {
        use crate::models::user::service::list::get_self_user_data;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_self_user_data(
            &logged_user_uuid,
            &get_set_language(cxt),
            conn,
        )
    }

    async fn show_tokens(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<Vec<UserToken>> {
        use crate::models::user::access::manage::show_user_tokens;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        show_user_tokens(
            &logged_user_uuid,
            conn,
        )
    }

    async fn get_token(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<Token> {
        use crate::models::user::access::manage::get_user_token;

        check_authorized(cxt)?;

        get_user_token(cxt)
    }

    async fn update_token(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<Token> {
        use crate::models::user::access::manage::update_user_token;

        check_authorized(cxt)?;

        update_user_token(cxt)
    }

    async fn decode_token(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<Claims> {
        use crate::models::user::access::manage::decode_user_token;

        check_authorized(cxt)?;

        decode_user_token(cxt)
    }

    async fn delete_token(
        &self,
        cxt: &Context<'_>,
        token: String
    ) -> ServiceResult<bool> {
        use crate::models::user::access::manage::delete_target_token;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_target_token(
            &logged_user_uuid,
            token.as_str(),
            conn,
        )
    }

    async fn delete_all_tokens(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<i32> {
        use crate::models::user::access::manage::delete_tokens;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_tokens(
            &logged_user_uuid,
            conn,
        )
    }

    async fn logout(&self, cxt: &Context<'_>) -> ServiceResult<String> {
        use crate::models::user::access::logout::logout_user;

        // removed user token
        logout_user(cxt)
    }

    async fn notifications(
        &self,
        cxt: &Context<'_>,
        select_ids: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowNotification>> {
        use crate::models::user::notification::service::list::get_notifications;
        let select_ids: Vec<i32>  = select_ids.unwrap_or_default();
        let limit: i64 = limit.unwrap_or(100) as i64;
        let offset: i64 = offset.unwrap_or(0) as i64;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_notifications(
            &logged_user_uuid,
            &select_ids,
            &limit,
            &offset,
            conn,
        )
    }
}
