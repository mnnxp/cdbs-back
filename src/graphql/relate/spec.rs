use async_graphql::{self, Context, Object};

use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::relate_ref::spec;
use crate::models::relate_ref::spec::model::SpecTranslateList;

#[derive(Default)]
pub struct SpecQuery;
// #[derive(Default)]
// pub struct SpecMutation;

#[Object]
impl SpecQuery {
    async fn specs(
        &self,
        cxt: &Context<'_>,
        spec_id: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        // authorization check
        crate::models::user::access::logged::check_authorized(cxt)?;

        let spec_id: Vec<i32> = spec_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let conn: &PooledConnection = &get_conn(cxt)?;

        spec::service::list::get_specs(
            spec_id,
            limit,
            offset,
            &crate::models::user::get_set_language(cxt),
            conn,
        )
    }
}

// #[Object]
// impl SpecMutation {
// }
