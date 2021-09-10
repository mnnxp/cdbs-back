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
        id_program: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Program>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_program: Vec<i32> = id_program.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        program::service::list::get_programs(cxt, id_program, limit, offset)
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

        crate::models::user::check_authorized(cxt)?;

        Ok(create_program(data, conn)?)
    }
}
