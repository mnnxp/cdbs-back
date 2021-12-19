use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::component::component_modification::util::get_component_by_modification;
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::relate_ref::file as file;
use crate::schema::file_to_modification::dsl as file_to_modification;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns pre-signed URLs (in wrapper DownloadFile)
/// to get files associated with component_modifications
pub(crate) fn get_component_modification_files(
    logged_user_uuid: &Uuid,
    target_modification_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(target_modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let target_file_uuids: Vec<Uuid> = file_to_modification::file_to_modification
        .filter(file_to_modification::modification_uuid.eq(target_modification_uuid))
        .select(file_to_modification::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get file: {:?}", err);
            ServiceError::InternalServerError
        })?;

    file::service::list::get_urls_by_files_uuids(
        &target_file_uuids,
        conn
    )
}
