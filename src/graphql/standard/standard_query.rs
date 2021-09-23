use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::standard;
use crate::models::standard::model::{ShowStandardShort, StandardAndRelatedData};
use crate::models::user;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct StandardQuery;

#[Object]
impl StandardQuery {
    async fn standards(
        &self,
        cxt: &Context<'_>,
        standards_uuids: Vec<Uuid>,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        // authorization check
        let logged_user_uuid = user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        standard::service::list::find_by_uuids(
            &logged_user_uuid,
            &standards_uuids,
            &crate::models::user::get_set_language(cxt),
            conn,
        )
    }

    async fn standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<StandardAndRelatedData> {
        // authorization check
        let logged_user_uuid = user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        standard::service::list::find_by_uuid(
            &standard_uuid,
            &logged_user_uuid,
            &crate::models::user::get_set_language(cxt),
            conn,
        )
    }
}
