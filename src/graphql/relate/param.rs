use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::param;
use crate::models::relate_ref::param::model::{IptParamTranslateListData, ParamTranslateList};

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
        crate::models::user::access::logged::check_authorized(cxt)?;

        let param_id: Vec<i32> = param_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let conn: &PooledConnection = &get_conn(cxt)?;

        param::service::list::get_params(
            param_id,
            limit,
            offset,
            &crate::models::user::get_set_language(cxt),
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
        use param::service::register::create_param;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::access::logged::check_authorized(cxt)?;

        create_param(data, conn)
    }
}
