use async_graphql::{self, Context, Object};

use super::attributes::IptPaginate;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::language::get_set_language;
use crate::models::relate_ref::param::service::register::create_parameters;
use crate::models::relate_ref::param::{
    model::{IptParamTranslateListData, ParamTranslateList},
    service::list::get_params,
    service::register::create_param,
};
use crate::models::search::order::Paginate;
use crate::models::user::access::logged::check_authorized;

#[derive(Default)]
pub struct ParamQuery;
#[derive(Default)]
pub struct ParamMutation;

#[Object]
impl ParamQuery {
    /// Returns a list of available parameters with a filter by IDs. If no parameter filter by ids is specified,
    /// then sampling is performed by all parameters taking into account the specified language.
    async fn params(
        &self,
        cxt: &Context<'_>,
        param_ids: Option<Vec<i32>>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        check_authorized(cxt)?; // authorization check
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_params(
            &param_ids.unwrap_or_default(),
            &get_set_language(cxt),
            &p,
            conn,
        )
    }
}

#[Object]
impl ParamMutation {
    /// Returns a ParamTranslateList structure of a new or existing parameter if an identical one is found
    async fn register_param(
        &self,
        cxt: &Context<'_>,
        args: IptParamTranslateListData,
    ) -> ServiceResult<ParamTranslateList> {
        check_authorized(cxt)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        create_param(&args, conn)
    }

    /// Returns an array with ParamTranslateList of new or existing parameters,
    /// new parameters are not created if identical ones are found
    async fn register_params_bulk(
        &self,
        cxt: &Context<'_>,
        args: Vec<IptParamTranslateListData>,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        check_authorized(cxt)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        create_parameters(&args, conn)
    }
}
