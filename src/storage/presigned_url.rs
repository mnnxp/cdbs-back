use crate::errors::ServiceResult;
use crate::storage::model::StorageAccess;
use crate::storage::s3::Aws;

/// Gets presigned url for target file by path
pub(crate) fn download_presigned_url(
    access_storage: &StorageAccess,
    path_file: &str,
) -> ServiceResult<String> {
    let aws_access = Aws::from(access_storage);

    let presigned_url = aws_access.put_download_signed_url(
        &access_storage.bucket,
        path_file,
        800_u64, // <-- todo!(temp solution)
    )?;

    debug!("Presigned url: {:#?}", presigned_url);

    // Ok(String::from("Temp string for presigned url for download"))
    Ok(presigned_url)
}

/// Gets presigned url for upload files to storage
pub(crate) fn upload_presigned_url(
    access_storage: &StorageAccess,
    path_file: &str,
) -> ServiceResult<String> {
    let aws_access = Aws::from(access_storage);

    let presigned_url = aws_access.get_upload_signed_url(
        &access_storage.bucket,
        path_file,
        800_u64, // <-- todo!(temp solution)
    )?;

    debug!("Presigned url for upload: {:#?}", presigned_url);

    // Ok(String::from("Temp string for presigned url for upload"))
    Ok(presigned_url)
}
