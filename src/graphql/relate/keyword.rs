use async_graphql::{self, Context, Object};
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::keyword::{
    model::{IptKeywordData, Keyword, IptKeywordArg, KeywordArg},
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
        args: Option<IptKeywordArg>,
    ) -> ServiceResult<Vec<Keyword>> {
        // authorization check
        check_authorized(cxt)?;

        let arguments = match args {
            Some(x) => KeywordArg::from(x),
            None => KeywordArg::default(),
        };

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_keywords(&arguments, conn)
    }
}

#[Object]
impl KeywordMutation {
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
