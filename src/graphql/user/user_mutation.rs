use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::user::access::password::IptUpdatePassword;
use crate::models::user::model::IptUpdateUserData;
use crate::models::user::certificate::model::{
    IptUserCertificateData, IptUpdateUserCertificateData, DelUserCertificateData
};
use crate::models::user::model::{IptUserData, SlimUser};
use crate::models::relate_ref::file::model::UploadFile;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Adds a new user. Required values: email address, username and password.
    async fn register_user(
        &self,
        cxt: &Context<'_>,
        args: IptUserData,
    ) -> ServiceResult<SlimUser> {
        use crate::models::user::service::register::create_user;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_user(
            &args,
            conn
        )
    }

    /// Deletes a user and associated data.
    async fn delete_user_data(
        &self,
        cxt: &Context<'_>,
        password: String,
    ) -> ServiceResult<bool> {
        use crate::models::user::service::delete::delete_user;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_user(
            &logged_user_uuid,
            password.as_bytes(),
            conn,
        )
    }

    /// Sets a new password for an authorized user.
    async fn put_update_password(
        &self,
        cxt: &Context<'_>,
        args: IptUpdatePassword,
    ) -> ServiceResult<bool> {
        use crate::models::user::access::password::change_password;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        change_password(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Sets a user's access type to user data for other users.
    async fn change_type_access_user(
        &self,
        cxt: &Context<'_>,
        new_type_access: i32,
    ) -> ServiceResult<bool> {
        use crate::models::user::access::update::change_access_type_user;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        change_access_type_user(
            &logged_user_uuid,
            &new_type_access,
            conn
        )
    }

    /// Updates the user's underlying data by UUID.
    /// Returns the number of successful changes or an error if all the specified data already exists.
    async fn put_user_update(
        &self,
        cxt: &Context<'_>,
        args: IptUpdateUserData,
    ) -> ServiceResult<usize> {
        use crate::models::user::service::update::update_user;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        update_user(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Updates the user avatar. Returns a structure with a pre-signed URL for uploading an image file.
    async fn upload_favicon(
        &self,
        cxt: &Context<'_>,
        filename: String,
    ) -> ServiceResult<UploadFile> {
        use crate::models::user::relate::favicon::update_favicon;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        update_favicon(
            &logged_user_uuid,
            &filename,
            conn
        )
    }

    /// Uploading a new user certificate. Returns a structure with a pre-signed URL for uploading a certificate file.
    async fn upload_user_certificate(
        &self,
        cxt: &Context<'_>,
        cert_data: IptUserCertificateData,
    ) -> ServiceResult<UploadFile> {
        use crate::models::user::certificate::service::add::add_certificate;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_certificate(
            &logged_user_uuid,
            &cert_data,
            conn
        )
    }

    /// Updates a user certificate description.
    /// Returns true if the change was successful, and false if the certificate description is already installed.
    async fn update_user_certificate(
        &self,
        cxt: &Context<'_>,
        args: IptUpdateUserCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::user::certificate::service::update::update_certificate_description;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        update_certificate_description(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes a user certificate.
    async fn delete_user_certificate(
        &self,
        cxt: &Context<'_>,
        args: DelUserCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::user::certificate::service::delete::del_certificate_description;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_certificate_description(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Adds a company to a authorized user's favorite list.
    async fn add_company_fav(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::company_fav::service::add::add_company_fav;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_company_fav(
            &logged_user_uuid,
            &company_uuid,
            conn,
        )
    }

    /// Removes a company from the authorized user's favorites list.
    async fn delete_company_fav(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::company_fav::service::delete::delete_company_fav;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_company_fav(
            &logged_user_uuid,
            &company_uuid,
            conn,
        )
    }

    /// Adds a component to a authorized user's favorite list.
    async fn add_component_fav(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::component_fav::service::add::add_component_fav;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_component_fav(
            &logged_user_uuid,
            &component_uuid,
            conn,
        )
    }

    /// Removes a component from the authorized user's favorites list.
    async fn delete_component_fav(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::component_fav::service::delete::delete_component_fav;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_component_fav(
            &logged_user_uuid,
            &component_uuid,
            conn,
        )
    }

    /// Adds a standard to a authorized user's favorite list.
    async fn add_standard_fav(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::standard_fav::service::add::add_standard_fav;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_standard_fav(
            &logged_user_uuid,
            &standard_uuid,
            conn,
        )
    }

    /// Removes a standard from the authorized user's favorites list.
    async fn delete_standard_fav(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::standard_fav::service::delete::delete_standard_fav;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_standard_fav(
            &logged_user_uuid,
            &standard_uuid,
            conn,
        )
    }

    /// Adds a user to a authorized user's favorite list.
    async fn add_user_fav(
        &self,
        cxt: &Context<'_>,
        user_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::user_fav::service::add::add_user_fav;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_user_fav(
            &logged_user_uuid,
            &user_uuid,
            conn,
        )
    }

    /// Removes a user from the authorized user's favorites list.
    async fn delete_user_fav(
        &self,
        cxt: &Context<'_>,
        user_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::user::user_fav::service::delete::delete_user_fav;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_user_fav(
            &logged_user_uuid,
            &user_uuid,
            conn,
        )
    }

    /// Sets a notification as read.
    async fn read_notifications(
        &self,
        cxt: &Context<'_>,
        notifications_ids: Vec<i32>,
    ) -> ServiceResult<usize> {
        use crate::models::user::notification::service::update::set_notifications_as_read;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        set_notifications_as_read(
            &logged_user_uuid,
            &notifications_ids,
            conn,
        )
    }

    /// Removes a notification for a user.
    async fn delete_notifications(
        &self,
        cxt: &Context<'_>,
        notifications_ids: Vec<i32>,
    ) -> ServiceResult<usize> {
        use crate::models::user::notification::service::delete::delete_notifications;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_notifications(
            &logged_user_uuid,
            &notifications_ids,
            conn,
        )
    }

    async fn logout(&self, cxt: &Context<'_>) -> ServiceResult<String> {
        use crate::models::user::access::logout::logout_user;

        // removed user token
        logout_user(cxt)
    }
}
