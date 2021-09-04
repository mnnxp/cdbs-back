use crate::errors::{ServiceResult, ServiceError};
use crate::database::PgConn;
use crate::models::user::model::TargetUser;
use crate::storage::model::UserStorageAccess;
use crate::storage::wrapper::create_access_data::get_new_storage_access;
use crate::storage::wrapper::authorize_account::update_authorized_storage ;
use crate::storage::backblaze::b2_download_file_by_id::b2_headers_file_by_id;
use crate::storage::backblaze::b2_types::FileHeaders;

/// Gets only the headers information of file
pub(crate) async fn get_header_file_by_id(
    target_user: TargetUser,
    file_id: String,
    pool: PgConn,
) -> ServiceResult<FileHeaders> {
    let conn = pool.get().unwrap();

    // search for the valid token for user in the database
    let mut user_storage_access = UserStorageAccess::get(&target_user.0, &conn);

    // if not found valid access data in the database,
    if user_storage_access.is_err() {
        // requested generate new token is  for key
        user_storage_access = update_authorized_storage (target_user.clone(), pool.clone()).await;
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
        Err(e) => return Err(e),
    };

    let b2_api_url = user_storage_access.api_url;
    let b2_authorization_token = user_storage_access.authorization_token;

    let file_headers = b2_headers_file_by_id(
        &b2_api_url,
        &b2_authorization_token,
        &file_id,
    ).await;

    debug!("Headers file: {:#?}", file_headers);
    match file_headers {
        Ok(file_h) => {
            Ok(file_h)
        },
        Err(e) => Err(ServiceError::BadRequest(e.to_string())),
    }
}
