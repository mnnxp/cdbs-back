use async_graphql::{self, Context, Object};

use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::relate_ref::spec::service::{
    list::get_specs,
    path::get_paths_specs,
};
use crate::models::relate_ref::spec::model::{
    SpecTranslateList, SpecPath
};
use crate::models::relate_ref::language::get_set_language;
use crate::models::user::access::logged::check_authorized;

#[derive(Default)]
pub struct SpecQuery;
// #[derive(Default)]
// pub struct SpecMutation;

#[Object]
impl SpecQuery {
    async fn specs(
        &self,
        cxt: &Context<'_>,
        spec_ids: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        // authorization check
        check_authorized(cxt)?;

        let spec_ids: Vec<i32> = spec_ids.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_specs(
            &spec_ids,
            &limit,
            &offset,
            &get_set_language(cxt),
            conn,
        )
    }

    async fn specs_paths(
        &self,
        cxt: &Context<'_>,
        spec_ids: Option<Vec<i32>>,
        split_char: Option<char>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<SpecPath>> {
        // authorization check
        check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let spec_ids: Vec<i32> = spec_ids.unwrap_or_default();
        let split_char: char = split_char.unwrap_or('/');
        let limit: i32 = limit.unwrap_or(50);
        let offset: i32 = offset.unwrap_or(0);

        get_paths_specs(
            &spec_ids,
            &split_char,
            &limit,
            &offset,
            &get_set_language(cxt),
            conn
        )
    }
}

// #[Object]
// impl SpecMutation {
// }
