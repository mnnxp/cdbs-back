use async_graphql::{self, Context, Object};

use crate::auth::token::logged::check_authorized;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::program::{
    model::{IptProgramData, Program},
    service::list::get_programs,
    service::register::create_program,
};
use crate::models::search::order::Paginate;

use super::attributes::IptPaginate;

#[derive(Default)]
pub struct ProgramQuery;
#[derive(Default)]
pub struct ProgramMutation;

#[Object]
impl ProgramQuery {
    /// Returns a list of available softwares with a filter by IDs.
    /// If a filter is not specified, then all existing ones are aggregated.
    async fn programs(
        &self,
        ctx: &Context<'_>,
        program_ids: Option<Vec<i32>>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<Program>> {
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_programs(&program_ids.unwrap_or_default(), &p, conn)
    }
}

#[Object]
impl ProgramMutation {
    /// Adds new software. Returns the software ID and name.
    async fn register_program(
        &self,
        ctx: &Context<'_>,
        args: IptProgramData,
    ) -> ServiceResult<Program> {
        check_authorized(ctx)?;

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        create_program(&args, conn)
    }
}
