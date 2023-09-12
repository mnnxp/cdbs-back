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
    /// Returns directory partitions.
    /// It is possible to specify the top level (parent) section from which the list of children will be generated.
    /// Regardless of specifying the top section, specifying a filter by section IDs is available.
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

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_specs(&arguments, &get_set_language(cxt), conn)
    }

    /// Returns directory partition paths by IDs.
    /// When creating a partition path, the specified separator or the default separator "/" is used.
    /// The value "deep_level" sets the depth limit to the parent section.
    async fn specs_paths(
        &self,
        cxt: &Context<'_>,
        args: Option<IptSpecPathArg>,
    ) -> ServiceResult<Vec<SpecPath>> {
        // authorization check
        check_authorized(cxt)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        let arguments: SpecPathArg = match args {
            Some(x) => SpecPathArg::from(x),
            None => SpecPathArg::default(),
        };

        get_paths_specs(&arguments, &get_set_language(cxt), conn)
    }

    /// Returns paths to directory sections searched for by name partition.
    /// When creating a partition path, the specified separator or the default separator "/" is used.
    /// The value "deep_level" sets the depth limit to the parent section.
    async fn search_specs(
        &self,
        cxt: &Context<'_>,
        args: IptSearchSpecArg,
    ) -> ServiceResult<Vec<SpecPath>> {
        // authorization check
        check_authorized(cxt)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        let arguments: SearchSpecArg = SearchSpecArg::from(args);

        search_specs_by_name(&arguments, &get_set_language(cxt), conn)
    }
}

// #[Object]
// impl SpecMutation {
// }
