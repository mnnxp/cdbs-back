use crate::errors::ServiceResult;
use crate::database::PgConn;
use crate::models::user::model::TargetUser;
use crate::models::relate_ref::file::model::SlimFile;
// use crate::storage::model::UserStorageAccess;
use crate::storage::wrapper::storage_access::get_user_storage_access;
use crate::storage::wrapper::presigned_url::get_presigned_url;
use uuid::Uuid;

pub(crate) async fn get_url_file_by_uuid(
    target_user: TargetUser,
    target_file_uuid: Uuid,
    pool: PgConn,
) -> ServiceResult<String> {
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    let show_file = SlimFile::get_file_by_uuid(
        &target_file_uuid,
        &conn,
    ).unwrap();

    let storage_access = get_user_storage_access(
        target_user,
        pool
    ).await?;

    Ok(get_presigned_url(
        storage_access,
        show_file.path_file,
    ).await?)
}
