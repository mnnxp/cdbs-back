use crate::errors::{ServiceResult, ServiceError};
use crate::storage::model::StorageAccess;
use crate::storage::s3::Aws;

/// Gets presigned url for target file by path
pub(crate) fn download_presigned_url(
    access_storage: &StorageAccess,
    path_file: &str,
) -> ServiceResult<String> {
    match Aws::from(access_storage).put_download_signed_url(
        &access_storage.bucket,
        path_file,
        800_u64, // <-- todo!(temp solution)
    ) {
        Ok(presigned_url) => {
            debug!("Presigned url for download: {:#?}", presigned_url);
            Ok(presigned_url)
        },
        Err(err) => {
            debug!("Failed make presign-url: {:#?}", err);
            Err(ServiceError::BadRequest("Failed make presign-url".to_string()))
        }
    }
}

/// Gets presigned url for upload files to storage
pub(crate) fn upload_presigned_url(
    access_storage: &StorageAccess,
    path_file: &str,
) -> ServiceResult<String> {
    match Aws::from(access_storage).get_upload_signed_url(
        &access_storage.bucket,
        path_file,
        800_u64, // <-- todo!(temp solution)
    ) {
        Ok(presigned_url) => {
            debug!("Presigned url for upload: {:#?}", presigned_url);
            Ok(presigned_url)
        },
        Err(err) => {
            debug!("Failed make presign-url: {:#?}", err);
            Err(ServiceError::BadRequest("Failed make presign-url".to_string()))
        }
    }
}
