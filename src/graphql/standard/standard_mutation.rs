use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::standard::model::{IptStandardData, IptUpdateStandardData, SlimStandard};
use crate::models::standard::spec::model::IptStandardSpecsData;
use crate::models::standard::keyword::model::IptStandardKeywordsData;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct StandardMutation;

#[Object]
impl StandardMutation {
    async fn register_standard(
        &self,
        cxt: &Context<'_>,
        data: IptStandardData,
    ) -> ServiceResult<SlimStandard> {
        use crate::models::standard::service::register::create_standard;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_standard(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn put_standard_update(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
        data: IptUpdateStandardData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::service::update::update_standard_data;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_standard_data(
            &logged_user_uuid,
            &standard_uuid,
            &data,
            conn
        )
    }

    async fn delete_standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<SlimStandard> {
        use crate::models::standard::service::delete::del_standard_data;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_standard_data(
            &logged_user_uuid,
            &standard_uuid,
            conn
        )
    }

    async fn add_standard_specs(
        &self,
        cxt: &Context<'_>,
        data: IptStandardSpecsData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::spec::service::add::add_standard_specs;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_specs(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_standard_specs(
        &self,
        cxt: &Context<'_>,
        data: IptStandardSpecsData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::spec::service::delete::del_standard_specs;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_standard_specs(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_standard_keywords(
        &self,
        cxt: &Context<'_>,
        data: IptStandardKeywordsData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::keyword::service::add::add_standard_keywords;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_keywords(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_standard_keywords(
        &self,
        cxt: &Context<'_>,
        data: IptStandardKeywordsData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::keyword::service::delete::del_standard_keywords;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_standard_keywords(
            &logged_user_uuid,
            &data,
            conn
        )
    }
}
