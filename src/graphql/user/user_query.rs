use crate::auth::jwt::model::Claims;
use crate::auth::token::logged::check_authorized;
use crate::auth::token::UserToken;
use crate::auth::AuthContext;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::handler::extract_client_domain;
use crate::graphql::relate::attributes::IptPaginate;
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::Paginate;
use crate::models::user::model::{
    IptGetUserArg, IptUsersArg, ShowUserAndRelatedData, ShowUserShort, SlimUser,
    UserAndRelatedData, UsersArg,
};
use crate::models::user::notification::model::ShowNotification;

use async_graphql::{self, Context, Object};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Returns brief information about users with filter by:
    /// UUIDs, user (UUID), subscribers, favorite (for self).
    async fn users(
        &self,
        cxt: &Context<'_>,
        args: Option<IptUsersArg>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        use crate::models::user::service::list::get_users;

        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();
        let arguments: UsersArg = match args {
            Some(x) => UsersArg::from(x),
            None => UsersArg::default(),
        };
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_users(
            &logged_user_uuid,
            &arguments,
            &p,
            &extract_client_domain(cxt),
            conn,
        )
    }

    /// Returns basic and associated user data by UUID.
    async fn user(
        &self,
        cxt: &Context<'_>,
        args: IptGetUserArg,
    ) -> ServiceResult<ShowUserAndRelatedData> {
        use crate::models::user::service::list::get_user_data;

        // authorization check (if there is no token, it returns the default token)
        let options = ExtraOptions::from_cxt(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_user_data(&args, &options, conn)
    }

    /// Returns a structure with basic information about the user (SlimUser).
    async fn myself(&self, cxt: &Context<'_>) -> ServiceResult<SlimUser> {
        use crate::models::user::service::list::get_self_slim_data;

        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_self_slim_data(&logged_user_uuid, conn)
    }

    /// Returns complete information about the authorized user.
    async fn self_data(&self, cxt: &Context<'_>) -> ServiceResult<UserAndRelatedData> {
        use crate::models::user::service::list::get_self_user_data;

        // authorization check
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_self_user_data(&options, conn)
    }

    /// Returns the active tokens of the authorized user.
    async fn show_tokens(&self, cxt: &Context<'_>) -> ServiceResult<Vec<UserToken>> {
        use crate::models::user::access::manage::show_user_tokens;

        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        show_user_tokens(&logged_user_uuid, conn)
    }

    /// Returns the token provider, username, user UUID, program ID for user,
    /// token issuance date, and token expiration date.
    async fn decode_token(&self, cxt: &Context<'_>) -> ServiceResult<Claims> {
        use crate::models::user::access::manage::decode_user_token;

        check_authorized(cxt)?;

        decode_user_token(cxt)
    }

    /// Returns true if current token is still valid (not expired or revoked)
    async fn is_token_valid(&self, cxt: &Context<'_>) -> ServiceResult<bool> {
        use crate::models::user::access::manage::check_token_valid;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        check_token_valid(cxt, conn)
    }

    /// Returns days until token expiration for current user
    async fn token_days_until_expiry(&self, cxt: &Context<'_>) -> ServiceResult<i64> {
        use crate::models::user::access::manage::get_token_days_until_expiry;
        get_token_days_until_expiry(cxt)
    }

    /// Returns an aggregated list of user notifications.
    async fn notifications(
        &self,
        cxt: &Context<'_>,
        notification_ids: Option<Vec<i32>>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowNotification>> {
        use crate::models::user::notification::service::list::get_notifications;
        let logged_user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_notifications(
            &logged_user_uuid,
            &notification_ids.unwrap_or_default(),
            &p,
            conn,
        )
    }
}
