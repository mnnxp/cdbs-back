use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::fileset_for_program::model::{
    FilesetProgram, FilesetProgramRelatedData
};
use crate::models::component::component_modification::util::get_component_by_modification;
use crate::models::component::access::util::check_access_component_for_user;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn get_modification_filesets(
    logged_user_uuid: &Uuid,
    target_modification_uuid: &Uuid,
    target_program_id: &Option<Vec<i32>>,
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<FilesetProgramRelatedData>> {
    use crate::schema::fileset_for_program::dsl::*;

    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(target_modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let mut query = fileset_for_program.into_boxed();

    match target_program_id {
        // add filter for target program
        Some(prog_id) if !prog_id.is_empty() => {
            query = query.filter(modification_uuid.eq(target_modification_uuid)
                .and(program_id.eq_any(prog_id)));
        },
        _ => {
            // get all filesets for target modification
            query = query.filter(modification_uuid.eq(target_modification_uuid))
        },
    }

    let filesets = match query
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<FilesetProgram>(conn) {
        Ok(filesets) => FilesetProgramRelatedData::for_filesets(&filesets, conn),
        Err(err) => {
            debug!("Error get filesets for programs: {:?}", err);
            return Err(ServiceError::BadRequest(
                "Error get filesets for programs".to_string()
            ))
        },
    };

    // debug!("Filesets: {:#?}", filesets);

    filesets.map_err(|err| {
        debug!("Fail get data about programs for filesets: {:?}", err);
        ServiceError::BadRequest(
            "Fail get data about programs for filesets".to_string()
        )
    })
}
