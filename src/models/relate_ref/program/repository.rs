use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::extension::model::Extension;
use crate::models::relate_ref::program::model::Program;
use crate::schema::program_ref::dsl as program_ref;
use diesel::prelude::*;

impl Program {
    pub(crate) fn get_program_by_id(
        target_program_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Program> {
        program_ref::program_ref
            .filter(program_ref::id.eq(target_program_id))
            .first::<Program>(conn)
            .map_err(|err| {
                debug!("Failed get program: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Gets programs for target extension
    pub(crate) fn get_program_for_ext(
        target_ext_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Program> {
        let target_program_id = Extension::get_program_id(
            target_ext_id,
            conn
        ).expect("Error get ext data");

        program_ref::program_ref
            .filter(program_ref::id.eq(target_program_id))
            .first::<Program>(conn)
            .map_err(|err| {
                debug!("Failed get program: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
