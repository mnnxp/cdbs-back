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

    let api_url = user_storage_access.api_url;
    let authorization_token = user_storage_access.authorization_token;
    let bucket_id = user_storage_access.bucket_id;

    let received_url = upload_presigned_url(
        &api_url,
        &authorization_token,
        &bucket_id,
    );

    debug!("Upload: {:#?}", received_url);
    match received_url {
        Ok(url) => Ok(url),
        Err(e) => Err(ServiceError::BadRequest(e.to_string())),
    }
}
