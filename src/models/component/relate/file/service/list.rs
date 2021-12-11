use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::{
    model::ComponentFilesArg,
    access::util::check_access_component_for_user,
};
use crate::models::relate_ref::file::{
    model::DownloadFile,
    service::list::get_urls_by_files_uuids,
};
use crate::schema::file_to_component::dsl as file_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns pre-signed URLs (in wrapper DownloadFile) to get files associated with components
pub(crate) fn get_component_files(
    logged_user_uuid: &Uuid,
    arguments: &ComponentFilesArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let ComponentFilesArg {
        component_uuid,
        files_uuids,
    } = arguments;

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        component_uuid,
        &need_access_level,
        conn
    )?;

    let target_files_uuids: Vec<Uuid> = match files_uuids.is_empty() {
        true => file_to_component::file_to_component
            .filter(file_to_component::component_uuid.eq(component_uuid))
            .select(file_to_component::file_uuid)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed get files for component: {:?}", err);
                ServiceError::InternalServerError
            })?,
        false => file_to_component::file_to_component
            .filter(file_to_component::component_uuid.eq(component_uuid)
            .and(file_to_component::file_uuid.eq_any(files_uuids)))
            .select(file_to_component::file_uuid)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed get files for component: {:?}", err);
                ServiceError::InternalServerError
            })?,
    };

    get_urls_by_files_uuids(
        &target_files_uuids,
        conn
    )
}
