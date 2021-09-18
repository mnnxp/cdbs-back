// use crate::errors::ServiceResult;
use crate::storage::model::StorageAccess;
// use crate::storage::model::FileHeaders;
use crate::storage::s3::Aws;
use crate::storage::s3_client::delete_object_by_path;

use rusoto_s3::S3Client;

/// Delete file in storage by path
pub(crate) async fn delete_object(
    access_storage: &StorageAccess,
    path_file: &str,
) -> bool {
    let aws_access = Aws::from(access_storage);

    let client = S3Client::from(&aws_access);

    delete_object_by_path(
        &client,
        &access_storage.bucket,
        path_file,
    ).await
}
