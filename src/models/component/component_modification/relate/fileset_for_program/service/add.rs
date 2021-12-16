use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::{
    component_modification::{
        fileset_for_program::model::{IptFilesetProgramData, InsertableFilesetProgram},
        util::get_component_by_modification,
    },
    access::util::check_access_component_for_user,
};
use crate::schema::fileset_for_program::dsl as fileset_for_program;
use diesel::prelude::*;
use uuid::Uuid;

/// Creating a new set of files for the program
/// if found uuid for modification and program return Ok(fileset_uuid)
pub(crate) fn create_modification_fileset(
    logged_user_uuid: &Uuid,
    arg: &IptFilesetProgramData,
    conn: &PgConnection
) -> ServiceResult<Uuid> {

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(&arg.modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let find_fileset = &fileset_for_program::fileset_for_program
        .filter(fileset_for_program::modification_uuid.eq(&arg.modification_uuid)
        .and(fileset_for_program::program_id.eq(&arg.program_id)))
        .select(fileset_for_program::uuid)
        .limit(1)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Not found target fileset_for_program: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match find_fileset.first() {
        Some(x) => {
            debug!("The modification has a set of files for this program: {:?}", x);
            Ok(*x)
        },
        None => {
            let data: InsertableFilesetProgram = arg.into();

            diesel::insert_into(fileset_for_program::fileset_for_program)
                .values(&data)
                .returning(fileset_for_program::uuid)
                .get_result::<Uuid>(conn)
                .map_err(|err| {
                    debug!("Failed insert fileset_for_program: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
