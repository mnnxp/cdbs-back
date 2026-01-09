use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::search::order::Paginate;
use crate::models::supplier_service::{
    access::util::check_access_service_for_user, file::repository::get_file_uuids_by_service_uuid,
    model::ServiceFilesArg,
};
use diesel::prelude::*;
use uuid::Uuid;

/// Returns the pre-signed URL and other information for downloading the service files
pub(crate) fn get_service_files(
    logged_user_uuid: &Uuid,
    args: &ServiceFilesArg,
    paginate: &Paginate,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_service_for_user(
        logged_user_uuid,
        &args.service_uuid,
        need_access_level,
        conn,
    )?;

    let target_file_uuids =
        get_file_uuids_by_service_uuid(&args.service_uuid, &args.file_uuids, conn)?;

    DownloadFile::get_by_file_uuids(&target_file_uuids, paginate, domain, conn)
}
