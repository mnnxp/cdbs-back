use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::errors::ServiceResult;
// use crate::database::{get_conn, PooledConnection};
use crate::models::standard;
use crate::models::standard::model::{ShowStandardShort, StandardAndRelatedData};
use crate::models::user;

#[derive(Default)]
pub struct StandardQuery;

#[Object]
impl StandardQuery {
    async fn standards(
        &self,
        cxt: &Context<'_>,
        standards_uuids: Vec<String>,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        // authorization check
        let logged_user_uuid = user::get_logged_user_uuid(cxt, true)?;

        let mut target_standards_uuids = Vec::new();
        for x in standards_uuids.iter() {
            target_standards_uuids.push(Uuid::parse_str(x).unwrap());
        }

        standard::service::list::find_by_uuids(cxt, &target_standards_uuids, &logged_user_uuid)
    }

    async fn standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: String,
    ) -> ServiceResult<StandardAndRelatedData> {
        // authorization check
        let logged_user_uuid = user::get_logged_user_uuid(cxt, true)?;

        standard::service::list::find_by_uuid(
            cxt,
            &Uuid::parse_str(&standard_uuid)?,
            &logged_user_uuid,
        )
    }
}
