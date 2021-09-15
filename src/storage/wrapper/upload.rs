use crate::errors::{ServiceResult, ServiceError};
use crate::storage::model::UserStorageAccess;
use crate::storage::wrapper::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// Gets url for upload file
pub(crate) fn get_url_upload_file(
    target_user: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<String> {
    // search for the valid token for user in the database
    let user_storage_access = UserStorageAccess::get(target_user, conn)?;

    let path_file = "file_path_test".to_string();

    let received_url = upload_presigned_url(
        &user_storage_access,
        &path_file,
    );

    debug!("Upload: {:#?}", received_url);
    match received_url {
        Ok(url) => Ok(url),
        Err(e) => Err(ServiceError::BadRequest(e.to_string())),
    }
}
