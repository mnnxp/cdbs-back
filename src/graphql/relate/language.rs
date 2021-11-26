use async_graphql::{self, Context, Object};
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::language::{
    model::Language,
    service::list::get_languages,
};

#[derive(Default)]
pub struct LanguageQuery;
// #[derive(Default)]
// pub struct LanguageMutation;

#[Object]
impl LanguageQuery {
    async fn languages(
        &self,
        cxt: &Context<'_>,
        lang_id: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Language>> {
        // authorization check
        check_authorized(cxt)?;

        let lang_id: Vec<i32> = lang_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_languages(&lang_id, &limit, &offset, conn)
    }
}

// #[Object]
// impl LanguageMutation {
// }
