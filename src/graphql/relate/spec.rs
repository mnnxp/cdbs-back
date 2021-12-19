use async_graphql::{self, Context, Object};

use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::relate_ref::spec::service::{
    list::get_specs,
    path::get_paths_specs,
    search::search_specs_by_name,
};
use crate::models::relate_ref::spec::model::{
    SpecTranslateList, SpecPath, IptSpecPathArg, SpecPathArg,
    IptSearchSpecArg, SearchSpecArg, IptSpecArg, SpecArg
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
        args: Option<IptSpecArg>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        // authorization check
        check_authorized(cxt)?;

        let arguments: SpecArg = match args {
            Some(x) => SpecArg::from(x),
            None => SpecArg::default(),
        };

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_specs(&arguments, &get_set_language(cxt), conn)
    }

    async fn specs_paths(
        &self,
        cxt: &Context<'_>,
        args: Option<IptSpecPathArg>,
    ) -> ServiceResult<Vec<SpecPath>> {
        // authorization check
        check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let arguments: SpecPathArg = match args {
            Some(x) => SpecPathArg::from(x),
            None => SpecPathArg::default(),
        };

        get_paths_specs(&arguments, &get_set_language(cxt), conn)
    }

    async fn search_specs(
        &self,
        cxt: &Context<'_>,
        args: IptSearchSpecArg,
    ) -> ServiceResult<Vec<SpecPath>> {
        // authorization check
        check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let arguments: SearchSpecArg = SearchSpecArg::from(args);

        search_specs_by_name(&arguments, &get_set_language(cxt), conn)
    }
}

// #[Object]
// impl SpecMutation {
// }
