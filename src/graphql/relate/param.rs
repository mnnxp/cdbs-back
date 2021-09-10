use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::param;
use crate::models::relate_ref::param::model::{IptParamTranslateListData, ParamTranslateList};
use crate::models::user;

#[derive(Default)]
pub struct ParamQuery;
#[derive(Default)]
pub struct ParamMutation;

#[Object]
impl ParamQuery {
    async fn param(
        &self,
        cxt: &Context<'_>,
        id_param: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_param: Vec<i32> = id_param.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        param::service::list::get_params(cxt, id_param, limit, offset)
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

        crate::models::user::check_authorized(cxt)?;

        Ok(create_param(data, conn)?)
    }
}
