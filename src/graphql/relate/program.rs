use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::program;
use crate::models::relate_ref::program::model::{IptProgramData, Program};
use crate::models::user;

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
        // authorization check
        // user::util::check_authorized(cxt)?;

        let program_id: Vec<i32> = program_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        program::service::list::get_programs(cxt, program_id, limit, offset)
    }
}

#[Object]
impl ProgramMutation {
    async fn register_program(
        &self,
        cxt: &Context<'_>,
        data: IptProgramData,
    ) -> ServiceResult<Program> {
        use program::service::register::create_program;
        let conn: &PooledConnection = &get_conn(cxt)?;

        user::check_authorized(cxt)?;

        Ok(create_program(data, conn)?)
    }
}
