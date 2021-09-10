use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::errors::ServiceResult;
// use crate::database::{get_conn, PooledConnection};
use crate::models::user as user;
use crate::models::standard::model::{StandardAndRelatedData, ShowStandardShort};
use crate::models::standard as standard;

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
            let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;

            let mut target_standards_uuids = Vec::new();
            for x in standards_uuids.iter() {
                target_standards_uuids.push(Uuid::parse_str(x).unwrap());
            }

            standard::service::list::find_by_uuids(
                cxt,
                &target_standards_uuids,
                &logged_uuid_user,
            )
        }

        async fn standard(
            &self,
            cxt: &Context<'_>,
            standard_uuid: String,
        ) -> ServiceResult<StandardAndRelatedData> {
            // authorization check
            let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;

            standard::service::list::find_by_uuid(
                cxt,
                &Uuid::parse_str(&standard_uuid)?,
                &logged_uuid_user,
            )
        }

}
