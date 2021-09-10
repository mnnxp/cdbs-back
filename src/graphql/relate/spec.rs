use async_graphql::{self, Context, Object};

use crate::errors::ServiceResult;
// use crate::database::{get_conn, PooledConnection};
use crate::models::user as user;
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
        id_spec: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_spec: Vec<i32> = id_spec.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        spec::service::list::get_specs(cxt, id_spec, limit, offset)
    }

}

// #[Object]
// impl SpecMutation {
// }
