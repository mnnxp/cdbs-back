use crate::errors::ServiceResult;
use crate::storage::model::UserStorageAccess;
use crate::storage::s3::Aws;

/// Gets presigned url for target file by path
pub(crate) fn download_presigned_url(
    access_storage: &UserStorageAccess,
    path_file: &str,
) -> ServiceResult<String> {
    let aws_access = Aws::new(
        &access_storage.application_key_id,
        &access_storage.application_key,
        &access_storage.authorization_token,
    )?;

    // let bucket = user_storage_access.bucket_id;
    let bucket = "cdbs-test"; // todo!(add bucket name in database)
    let expires = 86400_u64;

    let presigned_url = aws_access.put_download_signed_url(
        bucket,
        path_file,
        expires,
    )?;

    debug!("Presigned url: {:#?}", presigned_url);

    // Ok(String::from("Temp string for presigned url for download"))
    Ok(presigned_url)
}


/// Gets presigned url for upload files to storage
pub(crate) fn upload_presigned_url(
    access_storage: &UserStorageAccess,
    path_file: &str,
    // content_sha1: &str,
) -> ServiceResult<String> {
    let aws_access = Aws::new(
        &access_storage.application_key_id,
        &access_storage.application_key,
        &access_storage.authorization_token,
    )?;

    // let bucket = user_storage_access.bucket_id;
    let bucket = "cdbs-test"; // todo!(add bucket name in database)
    let expires = 86400_u64;

    let presigned_url = aws_access.get_upload_signed_url(
        bucket,
        path_file,
        expires,
        // content_sha1,
    )?;

    debug!("Presigned url for upload: {:#?}", presigned_url);

    // Ok(String::from("Temp string for presigned url for upload"))
    Ok(presigned_url)
}
