use async_graphql::{self, Context, Object};

use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::relate_ref::spec::service::{
    list::get_specs,
    path::collect_path_spec,
};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::relate_ref::language::get_set_language;

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

        get_specs(
            &spec_id,
            &limit,
            &offset,
            &get_set_language(cxt),
            conn,
        )
    }

    async fn spec_path(
        &self,
        cxt: &Context<'_>,
        spec_id: i32,
        split_char: Option<char>,
    ) -> ServiceResult<String> {
        // authorization check
        crate::models::user::access::logged::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let split_char: char = split_char.unwrap_or('/');

        collect_path_spec(
            &spec_id,
            &split_char,
            &get_set_language(cxt),
            conn
        )
    }
}

// #[Object]
// impl SpecMutation {
// }
