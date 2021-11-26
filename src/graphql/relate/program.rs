use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::program::{
    model::{IptProgramData, Program},
    service::list::get_programs,
    service::register::create_program,
};

#[derive(Default)]
pub struct ProgramQuery;
#[derive(Default)]
pub struct ProgramMutation;

#[Object]
impl ProgramQuery {
    async fn programs(
        &self,
        cxt: &Context<'_>,
        program_id: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Program>> {

        let program_id: Vec<i32> = program_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_programs(&program_id, &limit, &offset, conn)
    }
}

#[Object]
impl ProgramMutation {
    async fn register_program(
        &self,
        cxt: &Context<'_>,
        data: IptProgramData,
    ) -> ServiceResult<Program> {
        check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_program(&data, conn)
    }
}
