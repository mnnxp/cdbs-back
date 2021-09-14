use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::SlimFile;
use crate::storage::model::UserStorageAccess;
use crate::storage::wrapper::presigned_url::download_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn get_url_file_by_uuid(
    target_file_uuid: &Uuid,
    storage_access: &UserStorageAccess,
    conn: &PgConnection,
) -> ServiceResult<String> {
    let show_file = SlimFile::get_file_by_uuid(
        target_file_uuid,
        conn,
    ).unwrap();

    download_presigned_url(
        storage_access.to_owned(),
        show_file.path_file,
    )
}
