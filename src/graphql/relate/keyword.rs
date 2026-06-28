use crate::auth::token::logged::check_authorized;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::relate::attributes::IptPaginate;
use crate::models::relate_ref::keyword::{
    model::{IptKeywordData, Keyword},
    service::list::get_keywords,
    service::register::create_keyword,
};
use crate::models::search::order::Paginate;
use async_graphql::{self, Context, Object};

#[derive(Default)]
pub struct KeywordQuery;
#[derive(Default)]
pub struct KeywordMutation;

#[Object]
impl KeywordQuery {
    /// Returns keywords by IDs.
    /// If a filter for keywords is not specified, then all existing ones are aggregated.
    /// Keywords can be used for components and standards, as well as for companies.
    async fn keywords(
        &self,
        cxt: &Context<'_>,
        keyword_ids: Vec<i32>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<Keyword>> {
        // authorization check
        check_authorized(cxt)?;
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_keywords(&keyword_ids, &p, conn)
    }
}

#[Object]
impl KeywordMutation {
    /// Adds a new keyword.
    /// Returns an error with the keyword ID if it already exists.
    async fn register_keyword(
        &self,
        cxt: &Context<'_>,
        args: IptKeywordData,
    ) -> ServiceResult<Keyword> {
        // authorization check
        check_authorized(cxt)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_keyword(&args, conn)
    }
}
