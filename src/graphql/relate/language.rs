use async_graphql::{self, Context, Object};
use crate::graphql::relate::attributes::IptPaginate;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::search::order::Paginate;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::language::{
    model::Language,
    service::list::get_languages,
};

#[derive(Default)]
pub struct LanguageQuery;

#[Object]
impl LanguageQuery {
    /// Returns a list of available languages.
    async fn languages(
        &self,
        cxt: &Context<'_>,
        lang_ids: Option<Vec<i32>>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<Language>> {
        // authorization check
        check_authorized(cxt)?;
        let p = paginate.map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_languages(&lang_ids.unwrap_or_default(), &p, conn)
    }
}