use crate::errors::ServiceResult;
use crate::models::relate_ref::extension::model::Extension;
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

    pub fn get_programs_by_ids(
        target_programs_ids: &[i32],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Program>> {
        Ok(program_ref::program_ref
            .filter(program_ref::id.eq_any(target_programs_ids))
            .load::<Program>(conn)?)
    }

    /// Gets programs for target extension
    pub fn get_program_for_ext(
        target_ext_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Program> {
        let target_program_id = Extension::get_program_id(
            target_ext_id,
            conn
        ).expect("Error get ext data");

        Ok(program_ref::program_ref
            .filter(program_ref::id.eq(target_program_id))
            .first::<Program>(conn)?)
    }
}
