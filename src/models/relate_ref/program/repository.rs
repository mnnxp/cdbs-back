use crate::errors::ServiceResult;
use crate::models::relate_ref::cache::program_by_id;
use crate::models::relate_ref::extension::model::Extension;
use crate::models::relate_ref::program::model::Program;
use diesel::prelude::*;

impl Program {
    /// Gets programs for target extension
    pub(crate) fn get_program_for_ext(
        target_ext_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Program> {
        program_by_id(Extension::get_program_id(target_ext_id, conn)?)
    }
}
