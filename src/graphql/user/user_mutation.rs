use crate::database::{get_pool, get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::user::access::password::IptUpdatePassword;
use crate::models::user::model::IptUpdateUserData;
use crate::models::user::certificate::model::{
    IptUserCertificateData, IptUpdateUserCertificateData, DelUserCertificateData
};
use crate::models::user::company_fav::model::{CompanyFav, IptCompanyFavData};
use crate::models::user::component_fav::model::{ComponentFav, IptComponentFavData};
use crate::models::user::standard_fav::model::{StandardFav, IptStandardFavData};
use crate::models::user::user_fav::model::{UserFav, IptUserFavData};
use crate::models::user::model::{IptUserData, SlimUser};
use crate::models::relate_ref::file::model::UploadFile;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    // Add new user
    async fn register_user(
        &self,
        cxt: &Context<'_>,
        data: IptUserData,
    ) -> ServiceResult<SlimUser> {
        use crate::models::user::service::register::create_user;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_user(
            &data,
            conn
        )
    }

    // Delete user and relating data
    async fn delete_user_data(
        &self,
        cxt: &Context<'_>,
        password: String,
    ) -> ServiceResult<bool> {
        use crate::models::user::service::delete::delete_user;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_user(
            &logged_user_uuid,
            password.as_bytes(),
            conn,
        )
    }

    async fn put_update_password(
        &self,
        cxt: &Context<'_>,
        data: IptUpdatePassword,
    ) -> ServiceResult<bool> {
        use crate::models::user::access::password::change_password;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_password(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn change_type_access_user(
        &self,
        cxt: &Context<'_>,
        new_type_access: i32,
    ) -> ServiceResult<bool> {
        use crate::models::user::access::update::change_access_type_user;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        Ok(change_access_type_user(
            &logged_user_uuid,
            &new_type_access,
            conn
        ))
    }

    async fn put_user_update(
        &self,
        cxt: &Context<'_>,
        data: IptUpdateUserData,
    ) -> ServiceResult<i32> {
        use crate::models::user::service::update::update_user;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_user(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn upload_favicon(
        &self,
        cxt: &Context<'_>,
        filename: String,
    ) -> ServiceResult<String> {
        use crate::models::user::relate::favicon::update_favicon;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_favicon(&logged_user_uuid, &filename, conn)
    }

    async fn upload_user_certificate(
        &self,
        cxt: &Context<'_>,
        cert_data: IptUserCertificateData,
    ) -> ServiceResult<UploadFile> {
        use crate::models::user::certificate::service::add::add_certificate;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_certificate(
            &logged_user_uuid,
            &cert_data,
            conn
        )
    }

    /// Update user certificate description
    async fn update_user_certificate(
        &self,
        cxt: &Context<'_>,
        data: IptUpdateUserCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::user::certificate::service::update::update_certificate_description;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_certificate_description(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_user_certificate(
        &self,
        cxt: &Context<'_>,
        data: DelUserCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::user::certificate::service::delete::del_certificate_description;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let pool = get_pool(cxt)?;

        del_certificate_description(
            &logged_user_uuid,
            &data,
            &pool
        ).await
    }

    async fn add_company_fav(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<CompanyFav> {
        use crate::models::user::company_fav::service::add::add_company_fav;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_user_fav(
            IptUserFavData {
                user_favorite_uuid: user_uuid,
                user_follower_uuid: logged_user_uuid
            },
            conn,
        )
    }

    async fn read_notification(
        &self,
        cxt: &Context<'_>,
        notification_id: i32,
    ) -> ServiceResult<bool> {
        use crate::models::user::notification::service::update::notification_is_read;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        notification_is_read(
            &logged_user_uuid,
            &notification_id,
            conn,
        )
    }

    async fn delete_notification(
        &self,
        cxt: &Context<'_>,
        notification_ids: Vec<i32>,
    ) -> ServiceResult<i32> {
        use crate::models::user::notification::service::delete::delete_notification;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_notification(
            &logged_user_uuid,
            &notification_ids,
            conn,
        )
    }
}
