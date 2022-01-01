use crate::errors::ServiceResult;
use crate::models::relate_ref::file::access::check_file_owner_err;
use crate::models::relate_ref::file::model::DownloadFile;
use diesel::PgConnection;
use uuid::Uuid;

/// Gets presigned url for download file
/// with checking ownership for logged user
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

    DownloadFile::get_by_file_uuid(target_file_uuid, conn)
}
