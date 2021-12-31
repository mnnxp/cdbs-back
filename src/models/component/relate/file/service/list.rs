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
    args: &ComponentFilesArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &args.component_uuid,
        &need_access_level,
        conn
    )?;

    let mut query = file_to_component::file_to_component.into_boxed();

    query = match args.files_uuids.is_empty() {
        true => query.filter(file_to_component::component_uuid.eq(&args.component_uuid)),
        false => query.filter(file_to_component::component_uuid.eq(&args.component_uuid)
            .and(file_to_component::file_uuid.eq_any(&args.files_uuids)))
    };

    let target_files_uuids: Vec<Uuid> = query
        .select(file_to_component::file_uuid)
        .limit(args.limit as i64)
        .offset(args.offset as i64)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get files for component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    get_urls_by_files_uuids(&target_files_uuids, conn )
}
