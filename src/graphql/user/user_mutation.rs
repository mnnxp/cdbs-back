use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::certificate::model::IptUserCertificateData;
use crate::models::user::company_fav::model::{CompanyFav, IptCompanyFavData};
use crate::models::user::component_fav::model::{ComponentFav, IptComponentFavData};
use crate::models::user::standard_fav::model::{StandardFav, IptStandardFavData};
use crate::models::user::user_fav::model::{UserFav, IptUserFavData};
use crate::models::user::model::{IptUserData, SlimUser};
use crate::models::user::notification::model::{Notification, NotificationData, SlimNotification};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    // Add new user
    async fn register_user(&self, cxt: &Context<'_>, data: IptUserData) -> ServiceResult<SlimUser> {
        use crate::models::user::service::register::create_user;
        let conn: &PooledConnection = &get_conn(cxt)?;

        create_user(data, conn)
    }

    async fn upload_favicon(
        &self,
        cxt: &Context<'_>,
        filename: String,
    ) -> ServiceResult<String> {
        use crate::models::user::service::upload::favicon::update_favicon;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_favicon(&logged_user_uuid, &filename, conn)
    }

    async fn upload_user_certificate(
        &self,
        cxt: &Context<'_>,
        cert_data: IptUserCertificateData,
    ) -> ServiceResult<String> {
        use crate::models::user::certificate::service::add::add_certificate;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_certificate(
            &logged_user_uuid,
            &cert_data,
            conn
        )
    }

    async fn add_company_fav(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<CompanyFav> {
        use crate::models::user::company_fav::service::add::add_company_fav;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_company_fav(
            IptCompanyFavData {
                company_uuid,
                user_uuid: logged_user_uuid
            },
            conn,
        )
    }

    async fn delete_company_fav(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<CompanyFav> {
        use crate::models::user::company_fav::service::delete::delete_company_fav;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_company_fav(
            IptCompanyFavData {
                company_uuid,
                user_uuid: logged_user_uuid
            },
            conn,
        )
    }

    async fn add_component_fav(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<ComponentFav> {
        use crate::models::user::component_fav::service::add::add_component_fav;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_component_fav(
            IptComponentFavData {
                component_uuid,
                user_uuid: logged_user_uuid
            },
            conn,
        )
    }

    async fn delete_component_fav(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<ComponentFav> {
        use crate::models::user::component_fav::service::delete::delete_component_fav;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_component_fav(
            IptComponentFavData {
                component_uuid,
                user_uuid: logged_user_uuid
            },
            conn,
        )
    }

    async fn add_standard_fav(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<StandardFav> {
        use crate::models::user::standard_fav::service::add::add_standard_fav;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_fav(
            IptStandardFavData {
                standard_uuid,
                user_uuid: logged_user_uuid
            },
            conn,
        )
    }

    async fn delete_standard_fav(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<StandardFav> {
        use crate::models::user::standard_fav::service::delete::delete_standard_fav;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_standard_fav(
            IptStandardFavData {
                standard_uuid,
                user_uuid: logged_user_uuid
            },
            conn,
        )
    }

    async fn add_user_fav(
        &self,
        cxt: &Context<'_>,
        user_uuid: Uuid,
    ) -> ServiceResult<UserFav> {
        use crate::models::user::user_fav::service::add::add_user_fav;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_user_fav(
            IptUserFavData {
                user_favorite_uuid: user_uuid,
                user_follower_uuid: logged_user_uuid
            },
            conn,
        )
    }

    async fn delete_user_fav(
        &self,
        cxt: &Context<'_>,
        user_uuid: Uuid,
    ) -> ServiceResult<UserFav> {
        use crate::models::user::user_fav::service::delete::delete_user_fav;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_user_fav(
            IptUserFavData {
                user_favorite_uuid: user_uuid,
                user_follower_uuid: logged_user_uuid
            },
            conn,
        )
    }

    async fn register_notification(
        &self,
        cxt: &Context<'_>,
        data: NotificationData,
    ) -> ServiceResult<SlimNotification> {
        use crate::models::user::notification::service::register::create_notification;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_notification(data, logged_user_uuid, conn)
    }

    async fn delete_notification(
        &self,
        cxt: &Context<'_>,
        notification_id: i32,
    ) -> ServiceResult<Notification> {
        use crate::models::user::notification::service::delete::delete_notification;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_notification(
            logged_user_uuid,
            notification_id,
            conn,
        )
    }
}
