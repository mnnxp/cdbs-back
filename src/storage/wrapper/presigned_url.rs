use crate::errors::ServiceResult;
use crate::storage::model::UserStorageAccess;
// use crate::storage::s3::Aws;

/// Gets presigned url for target file by path
pub(crate) fn download_presigned_url(
    _access_storage: UserStorageAccess,
    _path_file: String,
) -> ServiceResult<String> {

    Ok(String::from("Temp string for presigned url for download"))
}


/// Gets presigned url for upload files to storage
pub(crate) fn upload_presigned_url(
    _api_url: &str,
    _authorization_token: &str,
    _bucket_id: &str,
) -> ServiceResult<String> {

    Ok(String::from("Temp string for presigned url for upload"))
}
