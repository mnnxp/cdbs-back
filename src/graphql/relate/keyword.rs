use async_graphql::{self, Context, Object};
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::keyword::{
    model::{IptKeywordData, Keyword},
    service::list::get_keywords,
    service::register::create_keyword,
};

#[derive(Default)]
pub struct KeywordQuery;
#[derive(Default)]
pub struct KeywordMutation;

#[Object]
impl KeywordQuery {
    async fn keywords(
        &self,
        cxt: &Context<'_>,
        keyword_id: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Keyword>> {
        // authorization check
        check_authorized(cxt)?;

        let keyword_id: Vec<i32> = keyword_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_keywords(&keyword_id, &limit, &offset, conn)
    }
}

#[Object]
impl KeywordMutation {
    async fn register_keyword(
        &self,
        cxt: &Context<'_>,
        data: IptKeywordData,
    ) -> ServiceResult<Keyword> {
        // authorization check
        check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_keyword(&data, conn)
    }
}
