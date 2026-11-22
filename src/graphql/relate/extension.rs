use crate::auth::{require_admin, AuthContext};
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::extension::model::{Extension, IptExtensionData};
use crate::models::relate_ref::extension::service::register::create_extension;

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
        // Admin-only
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        require_admin(&logged_user_uuid)?;

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        create_extension(&args, conn)
    }
}
