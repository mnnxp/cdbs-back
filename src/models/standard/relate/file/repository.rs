use crate::errors::{ServiceResult, ServiceError};
use crate::models::search::order::{Paginate, Sort, TableName, objects_order};
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use crate::schema::file_to_standard::dsl as file_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFileRelatedData {
    /// Gets all files for standard by uuid without check for hide, delete etc
    pub(crate) fn for_standard_by_uuid(
        standard_uuid: &Uuid,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let object_uuids = get_file_uuids_by_standard_uuid(standard_uuid, &[], conn)?;
        let target_file_uuids: Vec<Uuid> = objects_order(
            &object_uuids,
            &Sort::parsing(TableName::FileRef, "", false),
            paginate,
            conn
        )?;
        ShowFileRelatedData::get_file_by_uuids(&target_file_uuids, conn)
    }
}

/// Returns an array of UUIDs of files relate with target standard
pub(crate) fn get_file_uuids_by_standard_uuid(
    standard_uuid: &Uuid,
    file_uuids: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let mut query = file_to_standard::file_to_standard.into_boxed();

    query = match file_uuids.is_empty() {
        true => query.filter(file_to_standard::standard_uuid.eq(standard_uuid)),
        false => query.filter(file_to_standard::standard_uuid.eq(standard_uuid)
            .and(file_to_standard::file_uuid.eq_any(file_uuids))),
    };

    query
        .select(file_to_standard::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get file_to_standard uuids: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Determines a standard UUID by a file UUID
pub(crate) fn get_standard_uuid_by_file_uuid(
    file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Option<Uuid>> {
    file_to_standard::file_to_standard
        .select(file_to_standard::standard_uuid)
        .filter(file_to_standard::file_uuid.eq(file_uuid))
        .first::<Uuid>(conn)
        .optional()
        .map_err(|err| {
            debug!("Failed get standard uuid by file uuid: {:?}", err);
            ServiceError::InternalServerError
        })
}