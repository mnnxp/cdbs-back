use crate::auth::token::logged::check_authorized;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::relate::attributes::IptPaginate;
use crate::models::relate_ref::language::{model::Language, service::list::get_languages};
use crate::models::search::order::Paginate;
use async_graphql::{self, Context, Object};

#[derive(Default)]
pub struct LanguageQuery;

#[Object]
impl LanguageQuery {
    /// Returns a list of available languages.
    async fn languages(
        &self,
        ctx: &Context<'_>,
        lang_ids: Option<Vec<i32>>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<Language>> {
        // authorization check
        check_authorized(ctx)?;
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_languages(&lang_ids.unwrap_or_default(), &p, conn)
    }
}
