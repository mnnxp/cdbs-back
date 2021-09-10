use async_graphql::{self, Context, Object};

use crate::errors::ServiceResult;
// use crate::database::{get_conn, PooledConnection};
use crate::models::relate_ref::language;
use crate::models::relate_ref::language::model::Language;
use crate::models::user;

#[derive(Default)]
pub struct LanguageQuery;
// #[derive(Default)]
// pub struct LanguageMutation;

#[Object]
impl LanguageQuery {
    async fn language(
        &self,
        cxt: &Context<'_>,
        id_lang: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Language>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_lang: Vec<i32> = id_lang.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        language::service::list::get_languages(cxt, id_lang, limit, offset)
    }
}

// #[Object]
// impl LanguageMutation {
// }
