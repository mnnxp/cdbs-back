use async_graphql::{self, Context, Object};
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::language::{
    model::{Language, IptLanguageArg, LanguageArg},
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
        args: Option<IptLanguageArg>,
    ) -> ServiceResult<Vec<Language>> {
        // authorization check
        check_authorized(cxt)?;

        let arguments = match args {
            Some(x) => LanguageArg::from(x),
            None => LanguageArg::default(),
        };

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_languages(&arguments, conn)
    }
}

// #[Object]
// impl LanguageMutation {
// }
