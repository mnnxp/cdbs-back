use async_graphql::{self, Context, Object};

use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
// use crate::models::user as user;
use crate::models::relate_ref::extension::model::{Extension, IptExtensionData};
use crate::models::relate_ref::extension as extension;

// #[derive(Default)]
// pub struct ExtensionQuery;
#[derive(Default)]
pub struct ExtensionMutation;

// #[Object]
// impl ExtensionQuery {
//
// }

#[Object]
impl ExtensionMutation {
async fn register_extension(
    &self,
    cxt: &Context<'_>,
    data: IptExtensionData,
) -> ServiceResult<Extension> {
    use extension::service::register::create_extension;
    let conn: &PooledConnection = &get_conn(cxt)?;

    crate::models::user::check_authorized(cxt)?;

    Ok(create_extension(data, conn)?)
}

}
