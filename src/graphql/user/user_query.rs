use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::jwt::model::{Claims, Token};
use crate::models::user::access::logged::{check_authorized, get_logged_user_uuid};
use crate::models::user::model::{
    ShowUserShort, SlimUser, UserAndRelatedData,
    ShowUserAndRelatedData, UsersArg, IptUsersArg, IptGetUserArg
};
use crate::models::user::notification::model::{ShowNotification, IptNotificationArg, NotificationArg};
use crate::models::user::access::model::UserToken;
use crate::models::relate_ref::language::get_set_language;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

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
    ) -> ServiceResult<Vec<ShowUserShort>> {
        use crate::models::user::service::list::get_users;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: UsersArg = match args {
            Some(x) => UsersArg::from(x),
            None => UsersArg::default(),
        };

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_users(&logged_user_uuid, &arguments, conn)
    }

    /// Returns basic and associated user data by UUID.
    async fn user(
        &self,
        cxt: &Context<'_>,
        args: IptGetUserArg,
    ) -> ServiceResult<ShowUserAndRelatedData> {
        use crate::models::user::service::list::get_user_data;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_user_data(
            &logged_user_uuid,
            &args,
            &get_set_language(cxt),
            conn
        )
    }

    /// Returns a structure with basic information about the user (SlimUser).
    async fn myself(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<SlimUser> {
        use crate::models::user::service::list::get_self_slim_data;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_self_slim_data(
            &logged_user_uuid,
            conn,
        )
    }

    /// Returns complete information about the authorized user.
    async fn self_data(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<UserAndRelatedData> {
        use crate::models::user::service::list::get_self_user_data;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_self_user_data(
            &logged_user_uuid,
            &get_set_language(cxt),
            conn,
        )
    }

    /// Returns the active tokens of the authorized user.
    async fn show_tokens(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<Vec<UserToken>> {
        use crate::models::user::access::manage::show_user_tokens;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        show_user_tokens(
            &logged_user_uuid,
            conn,
        )
    }

    /// Generates a token for the user without deleting other valid tokens.
    /// Returns the user's new authorization token.
    async fn get_token(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<Token> {
        use crate::models::user::access::manage::get_user_token;

        check_authorized(cxt)?;

        get_user_token(cxt)
    }

    /// Generates a token for the user with the user's other tokens deactivated.
    /// Returns the user's new authorization token.
    async fn update_token(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<Token> {
        use crate::models::user::access::manage::update_user_token;

        check_authorized(cxt)?;

        update_user_token(cxt)
    }

    /// Returns the token provider, username, user UUID, program ID for user,
    /// token issuance date, and token expiration date.
    async fn decode_token(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<Claims> {
        use crate::models::user::access::manage::decode_user_token;

        check_authorized(cxt)?;

        decode_user_token(cxt)
    }

    /// Deactivates the specified user token.
    async fn delete_token(
        &self,
        cxt: &Context<'_>,
        token: String
    ) -> ServiceResult<bool> {
        use crate::models::user::access::manage::delete_target_token;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_target_token(
            &logged_user_uuid,
            token.as_str(),
            conn,
        )
    }

    /// Deactivates all user tokens.
    async fn delete_all_tokens(
        &self,
        cxt: &Context<'_>
    ) -> ServiceResult<i32> {
        use crate::models::user::access::manage::delete_tokens;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_tokens(
            &logged_user_uuid,
            conn,
        )
    }

    /// Returns an aggregated list of user notifications.
    async fn notifications(
        &self,
        cxt: &Context<'_>,
        args: Option<IptNotificationArg>,
    ) -> ServiceResult<Vec<ShowNotification>> {
        use crate::models::user::notification::service::list::get_notifications;

        let arguments = match args {
            Some(x) => NotificationArg::from(x),
            None => NotificationArg::default(),
        };

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_notifications(&logged_user_uuid, &arguments, conn)
    }
}
