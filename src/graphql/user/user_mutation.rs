use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::certificate::model::IptUserCertificateData;
use crate::models::user::company_fav::model::{
    CompanyFav, IptCompanyFavData,
};
use crate::models::user::component_fav::model::{
    ComponentFav, IptComponentFavData,
};
use crate::models::user::standard_fav::model::{
    StandardFav, IptStandardFavData,
};
use crate::models::user::user_fav::model::{
    UserFav, IptUserFavData,
};
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

    async fn upload_favicon(
        &self,
        cxt: &Context<'_>,
        file_data: IptPreliminaryFileData,
    ) -> ServiceResult<String> {
        use crate::models::user::service::upload::favicon::update_favicon;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        Ok(update_favicon(&logged_user_uuid, &file_data, conn)?)
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

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        Ok(add_certificate(
            logged_user_uuid,
            cert_data,
            file_data,
            conn
        )?)
    }

    async fn add_company_fav(
        &self,
        cxt: &Context<'_>,
        company_uuid: String,
    ) -> ServiceResult<CompanyFav> {
        use crate::models::user::company_fav::service::add::add_company_fav;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let data = IptCompanyFavData {
            company_uuid: Uuid::parse_str(&company_uuid)?,
            user_uuid: logged_user_uuid
        };

        Ok(add_company_fav(
            data,
            conn
        )?)
    }

    async fn delete_company_fav(
        &self,
        cxt: &Context<'_>,
        company_uuid: String,
    ) -> ServiceResult<CompanyFav> {
        use crate::models::user::company_fav::service::delete::delete_company_fav;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let data = IptCompanyFavData {
            company_uuid: Uuid::parse_str(&company_uuid)?,
            user_uuid: logged_user_uuid
        };

        Ok(delete_company_fav(
            data,
            conn
        )?)
    }

    async fn add_component_fav(
        &self,
        cxt: &Context<'_>,
        component_uuid: String,
    ) -> ServiceResult<ComponentFav> {
        use crate::models::user::component_fav::service::add::add_component_fav;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let data = IptComponentFavData {
            component_uuid: Uuid::parse_str(&component_uuid)?,
            user_uuid: logged_user_uuid
        };

        Ok(add_component_fav(
            data,
            conn
        )?)
    }

    async fn delete_component_fav(
        &self,
        cxt: &Context<'_>,
        component_uuid: String,
    ) -> ServiceResult<ComponentFav> {
        use crate::models::user::component_fav::service::delete::delete_component_fav;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let data = IptComponentFavData {
            component_uuid: Uuid::parse_str(&component_uuid)?,
            user_uuid: logged_user_uuid
        };

        Ok(delete_component_fav(
            data,
            conn
        )?)
    }

    async fn add_standard_fav(
        &self,
        cxt: &Context<'_>,
        standard_uuid: String,
    ) -> ServiceResult<StandardFav> {
        use crate::models::user::standard_fav::service::add::add_standard_fav;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let data = IptStandardFavData {
            standard_uuid: Uuid::parse_str(&standard_uuid)?,
            user_uuid: logged_user_uuid
        };

        Ok(add_standard_fav(
            data,
            conn
        )?)
    }

    async fn delete_standard_fav(
        &self,
        cxt: &Context<'_>,
        standard_uuid: String,
    ) -> ServiceResult<StandardFav> {
        use crate::models::user::standard_fav::service::delete::delete_standard_fav;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let data = IptStandardFavData {
            standard_uuid: Uuid::parse_str(&standard_uuid)?,
            user_uuid: logged_user_uuid
        };

        Ok(delete_standard_fav(
            data,
            conn
        )?)
    }

    async fn add_user_fav(
        &self,
        cxt: &Context<'_>,
        user_uuid: String,
    ) -> ServiceResult<UserFav> {
        use crate::models::user::user_fav::service::add::add_user_fav;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let data = IptUserFavData {
            user_favorite_uuid: Uuid::parse_str(&user_uuid)?,
            user_follower_uuid: logged_user_uuid
        };

        Ok(add_user_fav(
            data,
            conn
        )?)
    }

    async fn delete_user_fav(
        &self,
        cxt: &Context<'_>,
        user_uuid: String,
    ) -> ServiceResult<UserFav> {
        use crate::models::user::user_fav::service::delete::delete_user_fav;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let data = IptUserFavData {
            user_favorite_uuid: Uuid::parse_str(&user_uuid)?,
            user_follower_uuid: logged_user_uuid
        };

        Ok(delete_user_fav(
            data,
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

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        Ok(create_notification(data, logged_user_uuid, conn)?)
    }

    async fn delete_notification(
        &self,
        cxt: &Context<'_>,
        notification_id: i32,
    ) -> ServiceResult<Notification> {
        use crate::models::user::notification::service::delete::delete_notification;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        Ok(delete_notification(
            logged_user_uuid,
            notification_id,
            conn,
        )?)
    }
}
