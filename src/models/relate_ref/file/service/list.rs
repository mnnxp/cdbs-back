use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::file::access::check_file_owner_err;
use crate::models::relate_ref::file::model::{SlimFile, DownloadFile};
use diesel::PgConnection;
use uuid::Uuid;

/// Gets presigned url for download target file
pub(crate) fn get_url_by_file_uuid(
    logged_user_uuid: &Uuid,
    target_file_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<DownloadFile> {
    // check ownership file
    check_file_owner_err(
        logged_user_uuid,
        target_file_uuid,
        conn
    )?;

    let slim_file = SlimFile::get_file_by_uuid(
        target_file_uuid,
        conn,
    ).unwrap();

    DownloadFile::get_by_slim_file(
        &slim_file,
        conn
    )
}

/// Gets presigned urls for target files by uuids
pub(crate) fn get_urls_by_files_uuids(
    target_file_uuids: &[Uuid],
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    // return error if not found files
    if target_file_uuids.is_empty() {
        return Err(ServiceError::BadRequest("Not found files".to_string()))
    }

    // get files data by uuids, return error if have fail
    let slim_files = match SlimFile::get_by_files_uuids(
        target_file_uuids,
        conn,
    ) {
        Ok(data) => data,
        Err(err) => {
            debug!("Fail get presigned url: {:?}", err);
            return Err(ServiceError::BadRequest("Fail get files data".to_string()))
        },
    };

    DownloadFile::get_by_slim_files(
        &slim_files,
        conn
    )
}
