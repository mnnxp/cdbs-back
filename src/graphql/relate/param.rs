use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::param::{
    model::{IptParamTranslateListData, ParamTranslateList, IptParamArg, ParamArg},
    service::list::get_params,
    service::register::create_param,
};
use crate::models::relate_ref::language::get_set_language;

#[derive(Default)]
pub struct ParamQuery;
#[derive(Default)]
pub struct ParamMutation;

#[Object]
impl ParamQuery {
    async fn params(
        &self,
        cxt: &Context<'_>,
        args: Option<IptParamArg>,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        check_authorized(cxt)?; // authorization check

        let arguments = match args {
            Some(x) => ParamArg::from(x),
            None => ParamArg::default(),
        };

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_params(&arguments, &get_set_language(cxt), conn)
    }
}

#[Object]
impl ParamMutation {
    async fn register_param(
        &self,
        cxt: &Context<'_>,
        args: IptParamTranslateListData,
    ) -> ServiceResult<i32> {
        check_authorized(cxt)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_param(&args, conn)
    }
}
