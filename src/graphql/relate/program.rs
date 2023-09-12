use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::program::{
    model::{IptProgramData, Program, IptProgramArg, ProgramArg},
    service::list::get_programs,
    service::register::create_program,
};

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
        cxt: &Context<'_>,
        args: Option<IptProgramArg>,
    ) -> ServiceResult<Vec<Program>> {
        let arguments = match args {
            Some(x) => ProgramArg::from(x),
            None => ProgramArg::default(),
        };

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_programs(&arguments, conn)
    }
}

#[Object]
impl ProgramMutation {
    /// Adds new software. Returns the software ID and name.
    async fn register_program(
        &self,
        cxt: &Context<'_>,
        args: IptProgramData,
    ) -> ServiceResult<Program> {
        check_authorized(cxt)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_program(&args, conn)
    }
}
