use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::extension;
use crate::models::relate_ref::extension::model::{Extension, IptExtensionData};

use async_graphql::{self, Context, Object};

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
    /// Creates an association of an extension with a software solution.
    async fn register_extension(
        &self,
        ctx: &Context<'_>,
        args: IptExtensionData,
    ) -> ServiceResult<Extension> {
        use extension::service::register::create_extension;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        crate::auth::token::logged::check_authorized(ctx)?;

        create_extension(&args, conn)
    }
}
