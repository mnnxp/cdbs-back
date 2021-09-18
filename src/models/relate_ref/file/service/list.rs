use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::SlimFile;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::download_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn get_url_file_by_uuid(
    _logged_user_uuid: &Uuid, // <-- todo!(access check)
    target_file_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<String> {
    let storage_access = StorageAccess::get(conn)?;

    let show_file = SlimFile::get_file_by_uuid(
        target_file_uuid,
        conn,
    ).unwrap();

    download_presigned_url(
        &storage_access,
        &show_file.path_file,
    )
}
