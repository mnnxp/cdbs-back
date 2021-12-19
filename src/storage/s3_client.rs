use crate::errors::ServiceResult;
use tokio::runtime::Runtime;
use rusoto_core::request::HttpClient;
use rusoto_s3::{HeadObjectOutput, S3, S3Client};
use rusoto_signature::credential::StaticProvider;

impl From<&super::s3::Aws> for S3Client {
    /// Get S3Client from Aws data
    fn from(aws_access: &super::s3::Aws) -> S3Client {
        S3Client::new_with(
            HttpClient::new().expect("Failed to creat HTTP client"),
            StaticProvider::from(aws_access.clone_credentials()),
            aws_access.clone_region(),
        )
    }
}

/// Get object headers
pub(crate) async fn get_object_headers_by_path(
    client: &S3Client,
    bucket: &str,
    path_file: &str,
) -> ServiceResult<HeadObjectOutput> {
    let req = rusoto_s3::HeadObjectRequest{
        bucket: bucket.to_string(),
        key: path_file.to_string(),
        ..Default::default()
    };

    debug!("HeadObjectRequest: {:#?}", req);

    // Create the runtime
    let rt = Runtime::new().unwrap();

    // Spawn a future onto the runtime
    let res = rt.block_on(async {
        // trying to get headers
        client.head_object(req).await
        // debug!("future: {:?}", future);
    });

    match res {
        Ok(x) => {
            debug!("HeadObjectOutput: {:#?}", x);
            Ok(x)
        },
        Err(err) => {
            debug!("Err HeadObjectOutput: {:#?}", err);
            Err(crate::errors::ServiceError::BadRequest("fail get header".to_string()))
        },
    }
}

/// Delete object by path
pub(crate) async fn delete_object_by_path(
    client: &S3Client,
    bucket: &str,
    path_file: &str,
) -> bool {
    let req = rusoto_s3::DeleteObjectRequest {
        bucket: bucket.to_string(),
        key: path_file.to_string(),
        ..Default::default()
    };

    // self.client.as_ref().unwrap().delete_object

    debug!("DeleteObjectRequest: {:#?}", req);

    // Create the runtime
    let rt = Runtime::new().unwrap();

    // Spawn a future onto the runtime
    let res = rt.block_on(async {
        // trying to get headers
        client.delete_object(req).await
        // debug!("future: {:?}", future);
    });

    match res {
        Ok(x) => {
            debug!("DeleteObjectRequest: {:#?}", x);
            true
        },
        Err(err) => {
            debug!("Err DeleteObjectRequest: {:#?}", err);
            false
        },
    }
}
