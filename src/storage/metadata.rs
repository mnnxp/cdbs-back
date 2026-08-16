use crate::config::{aws_client, s3_bucket};
use crate::errors::ServiceResult;
use crate::storage::model::FileHeaders;
use crate::storage::s3_client::get_object_headers_by_path;

use rusoto_s3::S3Client;

/// Gets files headers in storage
pub(crate) async fn object_headers(path_file: &str) -> ServiceResult<FileHeaders> {
    let client = S3Client::from(&aws_client());

    let file_headers = get_object_headers_by_path(&client, s3_bucket(), path_file).await?;

    FileHeaders::from_head_object(file_headers)
}
