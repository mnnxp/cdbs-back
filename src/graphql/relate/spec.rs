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
use crate::models::search::order::Paginate;
use crate::models::user::access::logged::check_authorized;

use super::attributes::IptPaginate;

#[Object]
impl SpecTranslateList {
    /// Catalog element identifier
    async fn spec_id(&self) -> &i32 {
        &self.spec_id
    }

    /// Name localization language identifier
    async fn lang_id(&self) -> &i32 {
        &self.lang_id
    }

    /// Localized catalog name
    async fn spec(&self) -> &String {
        &self.spec
    }

    /// Localized catalog name
    async fn parent_spec(&self, cxt: &Context<'_>) -> SpecTranslateList {
        // return itself if this is the root catalog
        if self.spec_id == 1 { return self.clone() }
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        SpecTranslateList::get_parent_by_id(&self.spec_id, &self.lang_id, conn)
            .expect("Error loading parent spec")
    }
}

#[derive(Default)]
pub struct SpecQuery;

#[Object]
impl SpecQuery {
    /// Returns catalogs.
    /// It is possible to specify the top level (parent) section from which the list of children will be generated.
    /// Regardless of specifying the top section, specifying a filter by section IDs is available.
    async fn specs(
        &self,
        cxt: &Context<'_>,
        args: Option<IptSpecArg>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        // authorization check
        check_authorized(cxt)?;
        let arguments: SpecArg = match args {
            Some(x) => SpecArg::from(x),
            None => SpecArg::default(),
        };
        let p = paginate.map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_specs(&arguments, &get_set_language(cxt), &p, conn)
    }

    /// Returns catalogs paths by IDs.
    /// When creating a catalog path, the specified separator or the default separator "/" is used.
    /// The value "deep_level" sets the depth limit to the parent section.
    async fn specs_paths(
        &self,
        cxt: &Context<'_>,
        args: Option<IptSpecPathArg>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<SpecPath>> {
        // authorization check
        check_authorized(cxt)?;
        let arguments: SpecPathArg = match args {
            Some(x) => SpecPathArg::from(x),
            None => SpecPathArg::default(),
        };
        let p = paginate.map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_paths_specs(&arguments, &get_set_language(cxt), &p, conn)
    }

    /// Returns paths to directory sections searched for by name catalog.
    /// When creating a catalog path, the specified separator or the default separator "/" is used.
    /// The value "deep_level" sets the depth limit to the parent section.
    async fn search_specs(
        &self,
        cxt: &Context<'_>,
        args: IptSearchSpecArg,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<SpecPath>> {
        // authorization check
        check_authorized(cxt)?;
        let arguments: SearchSpecArg = SearchSpecArg::from(args);
        let p = paginate.map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        search_specs_by_name(&arguments, &get_set_language(cxt), &p, conn)
    }
}
