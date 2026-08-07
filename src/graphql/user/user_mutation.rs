use crate::auth::api_key::model::IptUpdateApiKeyData;
use crate::auth::api_key::repository::{
    change_api_key, generate_api_key, regenerate_api_key, revoke_api_key,
};
use crate::auth::token::logged::check_authorized;
use crate::auth::token::model::Token;
use crate::auth::AuthContext;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::handler::extract_client_domain;
use crate::models::relate_ref::file::model::UploadFile;
use crate::models::user::access::password::IptUpdatePassword;
use crate::models::user::certificate::model::{
    DelUserCertificateData, IptUpdateUserCertificateData, IptUserCertificateData,
};
use crate::models::user::model::IptUpdateUserData;
use crate::models::user::model::{IptUserData, SlimUser};

use async_graphql::{self, Context, Object};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Adds a new user. Required values: email address, username and password.
    async fn register_user(&self, ctx: &Context<'_>, args: IptUserData) -> ServiceResult<SlimUser> {
        use crate::models::user::service::register::create_user;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        create_user(args, conn)
    }

    /// Deletes a user and associated data.
    async fn delete_user_data(&self, ctx: &Context<'_>, password: String) -> ServiceResult<bool> {
        use crate::models::user::service::delete::delete_user;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        delete_user(&logged_user_uuid, password.as_bytes(), conn)
    }

    /// Sets a new password for an authorized user.
    async fn put_update_password(
        &self,
        ctx: &Context<'_>,
        args: IptUpdatePassword,
    ) -> ServiceResult<bool> {
        use crate::models::user::access::password::change_password;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        change_password(&logged_user_uuid, &args, conn)
    }

    /// Sets a user's access type to user data for other users.
    async fn change_type_access_user(
        &self,
        ctx: &Context<'_>,
        new_type_access: i32,
    ) -> ServiceResult<bool> {
        use crate::models::user::access::update::change_access_type_user;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        change_access_type_user(&logged_user_uuid, new_type_access, conn)
    }

    /// Updates the user's underlying data by UUID.
    /// Returns the number of successful changes or an error if all the specified data already exists.
    async fn put_user_update(
        &self,
        ctx: &Context<'_>,
        args: IptUpdateUserData,
    ) -> ServiceResult<usize> {
        use crate::models::user::service::update::update_user;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        update_user(&logged_user_uuid, &args, conn)
    }

    /// Updates the user avatar. Returns a structure with a pre-signed URL for uploading an image file.
    async fn upload_favicon(
        &self,
        ctx: &Context<'_>,
        filename: String,
    ) -> ServiceResult<UploadFile> {
        use crate::models::user::relate::favicon::update_favicon;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        update_favicon(
            &logged_user_uuid,
            &filename,
            &extract_client_domain(ctx),
            conn,
        )
    }

    /// Uploading a new user certificate. Returns a structure with a pre-signed URL for uploading a certificate file.
    async fn upload_user_certificate(
        &self,
        ctx: &Context<'_>,
        cert_data: IptUserCertificateData,
    ) -> ServiceResult<UploadFile> {
        use crate::models::user::certificate::service::add::add_certificate;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        add_certificate(
            &logged_user_uuid,
            &cert_data,
            &extract_client_domain(ctx),
            conn,
        )
    }

    /// Updates a user certificate description.
    /// Returns true if the change was successful, and false if the certificate description is already installed.
    async fn update_user_certificate(
        &self,
        ctx: &Context<'_>,
        args: IptUpdateUserCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::user::certificate::service::update::update_certificate_description;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        update_certificate_description(&logged_user_uuid, &args, conn)
    }

    /// Removes a user certificate.
    async fn delete_user_certificate(
        &self,
        ctx: &Context<'_>,
        args: DelUserCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::user::certificate::service::delete::del_certificate;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        del_certificate(&logged_user_uuid, &args, conn)
    }

    /// Adds a company to a authorized user's favorite list.
    async fn add_company_fav(&self, ctx: &Context<'_>, company_uuid: Uuid) -> ServiceResult<bool> {
        use crate::models::user::company_fav::service::add::add_company_fav;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        add_company_fav(&logged_user_uuid, &company_uuid, conn)
    }

    /// Removes a company from the authorized user's favorites list.
    async fn delete_company_fav(
        &self,
        ctx: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::company_fav::service::delete::delete_company_fav;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        delete_company_fav(&logged_user_uuid, &company_uuid, conn)
    }

    /// Adds a component to a authorized user's favorite list.
    async fn add_component_fav(
        &self,
        ctx: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::component_fav::service::add::add_component_fav;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        add_component_fav(&logged_user_uuid, &component_uuid, conn)
    }

    /// Removes a component from the authorized user's favorites list.
    async fn delete_component_fav(
        &self,
        ctx: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::component_fav::service::delete::delete_component_fav;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        delete_component_fav(&logged_user_uuid, &component_uuid, conn)
    }

    /// Adds a standard to a authorized user's favorite list.
    async fn add_standard_fav(
        &self,
        ctx: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::standard_fav::service::add::add_standard_fav;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        add_standard_fav(&logged_user_uuid, &standard_uuid, conn)
    }

    /// Removes a standard from the authorized user's favorites list.
    async fn delete_standard_fav(
        &self,
        ctx: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::standard_fav::service::delete::delete_standard_fav;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        delete_standard_fav(&logged_user_uuid, &standard_uuid, conn)
    }

    /// Adds a user to a authorized user's favorite list.
    async fn add_user_fav(&self, ctx: &Context<'_>, user_uuid: Uuid) -> ServiceResult<bool> {
        use crate::models::user::user_fav::service::add::add_user_fav;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        add_user_fav(&logged_user_uuid, &user_uuid, conn)
    }

    /// Removes a user from the authorized user's favorites list.
    async fn delete_user_fav(&self, ctx: &Context<'_>, user_uuid: Uuid) -> ServiceResult<bool> {
        use crate::models::user::user_fav::service::delete::delete_user_fav;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        delete_user_fav(&logged_user_uuid, &user_uuid, conn)
    }

    /// Sets a notification as read.
    async fn read_notifications(
        &self,
        ctx: &Context<'_>,
        notifications_ids: Vec<i32>,
    ) -> ServiceResult<usize> {
        use crate::models::user::notification::service::update::set_notifications_as_read;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        set_notifications_as_read(&logged_user_uuid, &notifications_ids, conn)
    }

    /// Removes a notification for a user.
    async fn delete_notifications(
        &self,
        ctx: &Context<'_>,
        notifications_ids: Vec<i32>,
    ) -> ServiceResult<usize> {
        use crate::models::user::notification::service::delete::delete_notifications;

        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        delete_notifications(&logged_user_uuid, &notifications_ids, conn)
    }

    // TOKEN MANAGEMENT

    /// Generates a token for the user without deleting other valid tokens.
    /// Returns the user's new authorization token.
    async fn get_token(&self, ctx: &Context<'_>) -> ServiceResult<Token> {
        use crate::models::user::access::manage::get_user_token;
        check_authorized(ctx)?;
        get_user_token(ctx)
    }

    /// Generates a token for the user with the user's other tokens deactivated.
    /// Returns the user's new authorization token.
    async fn update_token(&self, ctx: &Context<'_>) -> ServiceResult<Token> {
        use crate::models::user::access::manage::update_user_token;
        check_authorized(ctx)?;
        update_user_token(ctx)
    }

    /// Deactivates the specified user token.
    async fn delete_token(&self, ctx: &Context<'_>, token: String) -> ServiceResult<bool> {
        use crate::models::user::access::manage::delete_target_token;
        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        delete_target_token(&logged_user_uuid, token.as_str(), conn)
    }

    /// Deactivates all user tokens.
    async fn delete_all_tokens(&self, ctx: &Context<'_>) -> ServiceResult<usize> {
        use crate::models::user::access::manage::delete_tokens;
        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        delete_tokens(&logged_user_uuid, conn)
    }

    async fn logout(&self, ctx: &Context<'_>) -> ServiceResult<String> {
        use crate::models::user::access::logout::logout_user;

        // removed user token
        logout_user(ctx)
    }

    // API KEY MANAGEMENT

    /// Creates a new API key for the authenticated user.
    async fn create_api_key(
        &self,
        ctx: &Context<'_>,
        name: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> ServiceResult<String> {
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        generate_api_key(&logged_user_uuid, &name, expires_at, conn)
    }

    /// Updates an API key metadata.
    async fn update_api_key(
        &self,
        ctx: &Context<'_>,
        key_id: i32,
        args: IptUpdateApiKeyData,
    ) -> ServiceResult<usize> {
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        change_api_key(key_id, &logged_user_uuid, &args, conn)
    }

    /// Deletes (revokes) an API key.
    async fn delete_api_key(&self, ctx: &Context<'_>, key_id: i32) -> ServiceResult<bool> {
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        revoke_api_key(key_id, &logged_user_uuid, conn)
    }

    /// Regenerates an API key (revokes old, creates new).
    async fn rotate_api_key(&self, ctx: &Context<'_>, key_id: i32) -> ServiceResult<String> {
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        regenerate_api_key(key_id, &logged_user_uuid, conn)
    }
}
