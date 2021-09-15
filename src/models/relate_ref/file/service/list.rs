use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::SlimFile;
use crate::storage::model::UserStorageAccess;
use crate::storage::wrapper::presigned_url::download_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn get_url_file_by_uuid(
    logged_user_uuid: &Uuid,
    target_file_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<String> {
    let storage_access = UserStorageAccess::get(
        logged_user_uuid,
        conn
    )?;

    let show_file = SlimFile::get_file_by_uuid(
        target_file_uuid,
        conn,
    ).unwrap();

    download_presigned_url(
        &storage_access,
        &show_file.path_file,
    )
}
