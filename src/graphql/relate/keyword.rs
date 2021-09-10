use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::keyword;
use crate::models::relate_ref::keyword::model::{IptKeywordData, Keyword};
use crate::models::user;

#[derive(Default)]
pub struct KeywordQuery;
#[derive(Default)]
pub struct KeywordMutation;

#[Object]
impl KeywordQuery {
    async fn keywords(
        &self,
        cxt: &Context<'_>,
        id_keyword: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Keyword>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_keyword: Vec<i32> = id_keyword.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        keyword::service::list::get_keywords(cxt, id_keyword, limit, offset)
    }
}

#[Object]
impl KeywordMutation {
    async fn register_keyword(
        &self,
        cxt: &Context<'_>,
        data: IptKeywordData,
    ) -> ServiceResult<Keyword> {
        use keyword::service::register::create_keyword;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(create_keyword(data, conn)?)
    }
}
