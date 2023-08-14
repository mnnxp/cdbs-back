use crate::errors::ServiceResult;
use crate::models::ExtraOptions;
use crate::models::relate_ref::file::access::check_file_owner_err;
use crate::models::relate_ref::file::model::{DownloadFile, ShowFileRelatedData};
use diesel::PgConnection;
use uuid::Uuid;

/// Gets presigned url for download file
/// with checking ownership for logged user
pub(crate) fn get_url_by_file_uuid(
    logged_user_uuid: &Uuid,
    target_file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<DownloadFile> {
    // check ownership file
    check_file_owner_err(
        logged_user_uuid,
        target_file_uuid,
        conn
    )?;

    DownloadFile::get_by_file_uuid(target_file_uuid, conn)
}

/// Returns information about existing revisions of a file
pub(crate) fn get_revisions_by_file_uuid(
    file_uuid: &Uuid,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowFileRelatedData>> {
    // check ownership file
    check_file_owner_err(
        &options.logged_user_uuid,
        file_uuid,
        conn
    )?;

    ShowFileRelatedData::get_revisions_by_uuid(
        file_uuid,
        &options.limit,
        &options.offset,
        conn
    )
}
