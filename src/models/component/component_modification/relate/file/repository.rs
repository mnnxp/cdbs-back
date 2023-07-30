use crate::errors::{ServiceResult, ServiceError};
use crate::schema::file_to_modification::dsl as file_to_modification;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns an array of UUIDs of files relate with target component modification
/// with filter by file UUIDs
pub(crate) fn get_file_uuids_by_modification_uuid(
    modification_uuid: &Uuid,
    file_uuids: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let mut query = file_to_modification::file_to_modification.into_boxed();

    query = match file_uuids.is_empty() {
        true => query.filter(file_to_modification::modification_uuid.eq(modification_uuid)),
        false => query.filter(file_to_modification::modification_uuid.eq(modification_uuid)
            .and(file_to_modification::file_uuid.eq_any(file_uuids))),
    };

    query
        .select(file_to_modification::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get file_to_modification uuids: {:?}", err);
            ServiceError::InternalServerError
        })
}
