use crate::errors::{ServiceError, ServiceResult};
use crate::schema::modification_file_from_fileset::dsl as modification_file_from_fileset;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns an array of UUIDs of files relate with target component modification fileset
pub(crate) fn get_file_uuids_by_fileset_uuid(
    fileset_uuid: &Uuid,
    file_uuids: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let mut query = modification_file_from_fileset::modification_file_from_fileset.into_boxed();

    query = match file_uuids.is_empty() {
        true => query.filter(modification_file_from_fileset::fileset_uuid.eq(fileset_uuid)),
        false => query.filter(
            modification_file_from_fileset::fileset_uuid
                .eq(fileset_uuid)
                .and(modification_file_from_fileset::file_uuid.eq_any(file_uuids)),
        ),
    };

    query
        .select(modification_file_from_fileset::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get modification_file_from_fileset uuids: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Determines a modification fileset UUID by a file UUID
pub(crate) fn get_fileset_uuid_by_file_uuid(
    file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Option<Uuid>> {
    modification_file_from_fileset::modification_file_from_fileset
        .select(modification_file_from_fileset::fileset_uuid)
        .filter(modification_file_from_fileset::file_uuid.eq(file_uuid))
        .first::<Uuid>(conn)
        .optional()
        .map_err(|err| {
            debug!(
                "Failed get modification fileset uuid by file uuid: {:?}",
                err
            );
            ServiceError::InternalServerError
        })
}
