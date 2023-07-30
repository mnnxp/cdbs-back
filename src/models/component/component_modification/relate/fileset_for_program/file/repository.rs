use crate::errors::{ServiceResult, ServiceError};
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
        false => query.filter(modification_file_from_fileset::fileset_uuid.eq(fileset_uuid)
            .and(modification_file_from_fileset::file_uuid.eq_any(file_uuids))),
    };

    query
        .select(modification_file_from_fileset::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get modification_file_from_fileset uuids: {:?}", err);
            ServiceError::InternalServerError
        })
}
