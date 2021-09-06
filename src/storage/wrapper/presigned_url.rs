use crate::errors::ServiceResult;
use crate::storage::model::UserStorageAccess;
use crate::storage::s3::Aws;

/// Gets presigned url for target file by path
pub(crate) async fn get_presigned_url(
    access_storage: UserStorageAccess,
    path_file: String,
) -> ServiceResult<String> {

    let aws_access = Aws::new(
        &access_storage.application_key_id,
        &access_storage.application_key,
        &access_storage.authorization_token,
    ).await?;

    // let bucket = user_storage_access.bucket_id;
    let bucket = "cdbs-test".to_owned(); // todo!(add bucket name in database)
    let expires = 86400_u64;

    let presigned_url = aws_access.put_download_signed_url(
        bucket.as_str(),
        path_file.as_str(),
        expires,
    );

    debug!("Presigned url: {:#?}", presigned_url);

    match presigned_url {
        Ok(url) => {
            Ok(url)
        },
        Err(e) => Err(e),
    }
}
