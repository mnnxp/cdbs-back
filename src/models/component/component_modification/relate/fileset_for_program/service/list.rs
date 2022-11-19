use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::fileset_for_program::model::{
    FilesetProgramArg, FilesetProgram, FilesetProgramRelatedData
};
use crate::models::component::component_modification::util::get_component_by_modification;
use crate::models::component::access::util::check_access_component_for_user;
use crate::schema::fileset_for_program::dsl as fileset_for_program;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn get_modification_filesets(
    logged_user_uuid: &Uuid,
    arguments: &FilesetProgramArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<FilesetProgramRelatedData>> {
    let FilesetProgramArg {
        modification_uuid,
        program_ids,
        limit,
        offset,
    } = arguments;

    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let mut query = fileset_for_program::fileset_for_program.into_boxed();

    query = match program_ids.is_empty() {
        // get all filesets for target modification
        true => query
            .filter(fileset_for_program::modification_uuid.eq(modification_uuid)),
        // add filter for target program
        false => query
            .filter(fileset_for_program::modification_uuid.eq(modification_uuid)
            .and(fileset_for_program::program_id.eq_any(program_ids))),
    };

    let filesets = query.limit(*limit as i64)
        .offset(*offset as i64)
        .load::<FilesetProgram>(conn)
        .map_err(|err| {
            debug!("Error get filesets for programs: {:?}", err);
            ServiceError::InternalServerError
        })?;

    FilesetProgramRelatedData::for_filesets(&filesets, conn)
}
