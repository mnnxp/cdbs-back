use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::relate_ref::file as file;
use crate::schema::file_to_component::dsl as file_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns pre-signed URLs (in wrapper DownloadFile) to get files associated with components
pub(crate) fn get_component_files(
    _target_user_uuid: &Uuid, // <-- todo!(access check)
    target_component_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let target_file_uuids: Vec<Uuid> = file_to_component::file_to_component
        .filter(file_to_component::component_uuid.eq(target_component_uuid))
        .select(file_to_component::file_uuid)
        .load::<Uuid>(conn)?;

    file::service::list::get_urls_files_by_uuid(
        &target_file_uuids,
        conn
    )
}
