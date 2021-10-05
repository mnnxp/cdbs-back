use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::standard::model::{IptStandardData, IptUpdateStandardData, SlimStandard};

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
}
