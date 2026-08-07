use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::language::get_set_language;
use crate::models::relate_ref::spec::model::{
    IptSearchSpecArg, IptSpecArg, IptSpecPathArg, SearchSpecArg, SpecArg, SpecPath, SpecPathArg,
    SpecTranslateList,
};
use crate::models::relate_ref::spec::service::{
    list::get_specs, path::get_paths_specs, search::search_specs_by_name,
};
use crate::models::search::order::Paginate;

use super::attributes::IptPaginate;

#[Object]
impl SpecTranslateList {
    /// Catalog element identifier
    async fn spec_id(&self) -> i32 {
        self.spec_id
    }

    /// Name localization language identifier
    async fn lang_id(&self) -> i32 {
        self.lang_id
    }

    /// Localized catalog name
    async fn spec(&self) -> &String {
        &self.spec
    }

    /// Localized catalog name
    async fn parent_spec(&self, ctx: &Context<'_>) -> SpecTranslateList {
        // return itself if this is the root catalog
        if self.spec_id == 1 {
            return self.clone();
        }
        let conn: &mut PooledConnection = &mut get_conn(ctx).expect("Error get conn to DB");
        SpecTranslateList::get_parent_by_id(self.spec_id, self.lang_id, conn)
            .expect("Error loading parent spec")
    }
}

#[derive(Default)]
pub struct SpecQuery;

#[Object]
impl SpecQuery {
    /// Returns catalogs (specs). Token is not required.
    /// It is possible to specify the top level (parent) section from which the list of children will be generated.
    /// Regardless of specifying the top section, specifying a filter by section IDs is available.
    async fn specs(
        &self,
        ctx: &Context<'_>,
        args: Option<IptSpecArg>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let arguments: SpecArg = match args {
            Some(x) => SpecArg::from(x),
            None => SpecArg::default(),
        };
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_specs(&arguments, get_set_language(ctx), &p, conn)
    }

    /// Returns hierarchical paths for catalogs by their IDs.
    async fn specs_paths(
        &self,
        ctx: &Context<'_>,
        args: Option<IptSpecPathArg>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<SpecPath>> {
        let arguments: SpecPathArg = match args {
            Some(x) => SpecPathArg::from(x),
            None => SpecPathArg::default(),
        };
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_paths_specs(&arguments, get_set_language(ctx), &p, conn)
    }

    /// Returns hierarchical paths for catalogs found by name search.
    async fn search_specs(
        &self,
        ctx: &Context<'_>,
        args: IptSearchSpecArg,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<SpecPath>> {
        let arguments: SearchSpecArg = SearchSpecArg::from(args);
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        search_specs_by_name(&arguments, get_set_language(ctx), &p, conn)
    }
}
