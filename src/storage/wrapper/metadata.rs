use crate::errors::{ServiceResult, ServiceError};
use crate::storage::model::UserStorageAccess;
use crate::storage::backblaze::b2_download_file_by_id::b2_headers_file_by_id;
use crate::storage::backblaze::b2_types::FileHeaders;

/// Gets only the headers information of file
pub(crate) async fn get_headers_file_by_id(
    access_storage: &UserStorageAccess,
    file_id: &str,
) -> ServiceResult<FileHeaders> {
    let file_headers = b2_headers_file_by_id(
        &access_storage.api_url,
        &access_storage.authorization_token,
        file_id,
    ).await;

    debug!("Headers file: {:#?}", file_headers);
    match file_headers {
        Ok(file_h) => {
            Ok(file_h)
        },
        Err(e) => Err(ServiceError::BadRequest(e.to_string())),
    }
}
