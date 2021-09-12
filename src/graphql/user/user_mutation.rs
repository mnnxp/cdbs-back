use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::certificate::model::IptUserCertificateData;
use crate::models::user::model::{IptUserData, SlimUser};
use crate::models::user::notification::model::{Notification, NotificationData, SlimNotification};
use crate::models::relate_ref::file::model::IptPreliminaryFileData;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    // Add new user
    async fn register_user(&self, cxt: &Context<'_>, data: IptUserData) -> ServiceResult<SlimUser> {
        use crate::models::user::service::register::create_user;
        let conn: &PooledConnection = &get_conn(cxt)?;

        Ok(create_user(data, conn)?)
    }

    async fn upload_user_certificate(
        &self,
        cxt: &Context<'_>,
        cert_data: IptUserCertificateData,
        file_data: IptPreliminaryFileData,
    ) -> ServiceResult<String> {
        use crate::models::user::certificate::service::add::add_certificate;
        // let pool = get_pool(cxt)?;
        // let conn = pool.get().unwrap();
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(add_certificate(
            logged_uuid_user,
            cert_data,
            file_data,
            conn
        )?)
    }

    async fn register_notification(
        &self,
        cxt: &Context<'_>,
        data: NotificationData,
    ) -> ServiceResult<SlimNotification> {
        use crate::models::user::notification::service::register::create_notification;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(create_notification(data, logged_uuid_user, conn)?)
    }

    async fn delete_notification(
        &self,
        cxt: &Context<'_>,
        id_notification: i32,
    ) -> ServiceResult<Notification> {
        use crate::models::user::notification::service::delete::delete_notification;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(delete_notification(
            logged_uuid_user,
            id_notification,
            conn,
        )?)
    }
}
