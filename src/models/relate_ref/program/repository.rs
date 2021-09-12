use crate::errors::ServiceResult;
use crate::models::relate_ref::program::model::Program;
use crate::schema::program_ref::dsl as program_ref;
use diesel::prelude::*;

impl Program {
    pub fn get_program_by_id(
        target_program_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Program> {
        Ok(program_ref::program_ref
            .filter(program_ref::id.eq(target_program_id))
            .first::<Program>(conn)?)
    }

    pub fn get_program_by_vec_id(
        target_vec_program_id: &[i32],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Program>> {
        Ok(program_ref::program_ref
            .filter(program_ref::id.eq_any(target_vec_program_id))
            .load::<Program>(conn)?)
    }
}
