use crate::errors::{ServiceResult, ServiceError};
use crate::database::PgConn;
use crate::models::user::model::TargetUser;
use crate::storage::model::UserStorageAccess;
use crate::storage::wrapper::create_access_data::get_new_storage_access;
use crate::storage::wrapper::authorize_account::get_user_storage_access;
use crate::storage::backblaze::b2_get_upload_url::b2_get_upload_url;
use crate::storage::backblaze::b2_types::UploadUrlData;

/// Gets url for upload file
pub(crate) async fn get_url_upload_file(
    target_user: TargetUser,
    pool: PgConn,
) -> ServiceResult<UploadUrlData> {
    let conn = pool.get().unwrap();
    let mut user_storage_access = UserStorageAccess::get(&target_user.0, &conn);

    // if not found valid access data in the database,
    if user_storage_access.is_err() {
        // requested generate new token is  for key
        user_storage_access = get_user_storage_access(target_user.clone(), pool.clone()).await;
    }

    // if the access data could not be found in the database,
    if user_storage_access.is_err() {
        // is requested new key for user
        user_storage_access = get_new_storage_access(target_user, pool).await;
    }

    let user_storage_access = match user_storage_access {
        // data for update received
        Ok(new_access) => new_access,
        // fatality error
        Err(e) => return Err(ServiceError::BadRequest(e.to_string())),
    };

    let b2_api_url = user_storage_access.api_url;
    let b2_authorization_token = user_storage_access.authorization_token;
    let b2_bucket_id = user_storage_access.bucket_id;

    let x_url = b2_get_upload_url(
        &b2_api_url,
        &b2_authorization_token,
        &b2_bucket_id,
    ).await;

    debug!("Upload: {:#?}", x_url);
    match x_url {
        Ok(url) => Ok(url),
        Err(e) => Err(ServiceError::BadRequest(e.to_string())),
    }
}
