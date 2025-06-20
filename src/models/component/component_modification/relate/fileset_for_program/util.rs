use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::component::component_modification::util::get_component_by_modification;
use diesel::prelude::*;
use uuid::Uuid;

/// Get component uuid from fileset by uuid
pub(crate) fn get_component_by_fileset(
    target_fileset_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    // get component uuid
    get_component_by_modification(
        // get component modification uuid from fileset data
        &get_modification_by_fileset(target_fileset_uuid, conn)?,
        conn,
    )
}

/// Get modification uuid from fileset by uuid
pub(crate) fn get_modification_by_fileset(
    target_fileset_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    use crate::schema::fileset_for_program::dsl::*;
    fileset_for_program
        .filter(uuid.eq(target_fileset_uuid))
        .select(modification_uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get_modification_by_fileset data: {:?}", err);
            get_err_msg(ErrorMessage::NotFoundFilesetData)
        })
}
