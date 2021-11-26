use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::param::{
    model::{IptParamTranslateListData, ParamTranslateList},
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
        param_id: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        // authorization check
        check_authorized(cxt)?;

        let param_id: Vec<i32> = param_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_params(
            &param_id,
            &limit,
            &offset,
            &get_set_language(cxt),
            conn,
        )
    }
}

#[Object]
impl ParamMutation {
    async fn register_param(
        &self,
        cxt: &Context<'_>,
        data: IptParamTranslateListData,
    ) -> ServiceResult<ParamTranslateList> {
        check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_param(&data, conn)
    }
}
