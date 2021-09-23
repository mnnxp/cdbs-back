use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::standard::model::{IptStandardData, SlimStandard};

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
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        create_standard(logged_user_uuid, data, conn)
    }
}
