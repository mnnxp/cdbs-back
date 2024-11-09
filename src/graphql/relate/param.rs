use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::search::order::Paginate;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::param::{
    model::{IptParamTranslateListData, ParamTranslateList},
    service::list::get_params,
    service::register::create_param,
};
use crate::models::relate_ref::language::get_set_language;
use super::attributes::IptPaginate;

#[derive(Default)]
pub struct ParamQuery;
#[derive(Default)]
pub struct ParamMutation;

#[Object]
impl ParamQuery {
    /// Returns a list of available parameters with a filter by IDs.
    /// If a filter for parameter names is not specified, then all existing ones are aggregated.
    async fn params(
        &self,
        cxt: &Context<'_>,
        param_ids: Option<Vec<i32>>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        check_authorized(cxt)?; // authorization check
        let p = paginate.map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_params(&param_ids.unwrap_or_default(), &get_set_language(cxt), &p, conn)
    }
}

#[Object]
impl ParamMutation {
    /// Returns a ID of the parameter name.
    /// A new parameter is not registered if one already exists.
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
