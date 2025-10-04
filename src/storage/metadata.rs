use crate::errors::ServiceResult;
use crate::storage::model::FileHeaders;
use crate::storage::model::StorageAccess;
use crate::storage::s3::Aws;
use crate::storage::s3_client::get_object_headers_by_path;

use rusoto_s3::S3Client;

/// Gets files headers in storage
pub(crate) async fn object_headers(
    access_storage: &StorageAccess,
    path_file: &str,
) -> ServiceResult<FileHeaders> {
    let aws_access = Aws::from(access_storage);

    let client = S3Client::from(&aws_access);

    let file_headers =
        get_object_headers_by_path(&client, &access_storage.bucket, path_file).await?;

    Ok(file_headers.into())
}
