use crate::errors::ServiceResult;
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::relate_ref::file as file;
use crate::schema::file_to_component::dsl as file_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns pre-signed URLs (in wrapper DownloadFile) to get files associated with components
pub(crate) fn get_component_files(
    logged_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        target_component_uuid,
        &need_access_level,
        conn
    )?;

    let target_file_uuids: Vec<Uuid> = file_to_component::file_to_component
        .filter(file_to_component::component_uuid.eq(target_component_uuid))
        .select(file_to_component::file_uuid)
        .load::<Uuid>(conn)?;

    file::service::list::get_urls_files_by_uuid(
        &target_file_uuids,
        conn
    )
}
