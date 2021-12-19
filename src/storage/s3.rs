use crate::errors::ServiceResult;
use rusoto_signature::{
    Region,
    credential::AwsCredentials,
};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};

#[derive(Clone)]
pub(crate) struct Aws {
    credentials: AwsCredentials,
    region: Region
}

impl Aws {
    /// Create new Aws access
    pub(crate) fn new(
        access_key_id: &str,
        secret_access_key: &str,
        region: &str,
        endpoint: &str,
    ) -> Aws {
        let credentials = AwsCredentials::new(
            access_key_id,
            secret_access_key,
            None,
            None
        );

        // debug!("Credentials: {:#?}", credentials);

        let region = Region::Custom {
            name: region.to_string(),
            endpoint: endpoint.to_string(),
        };

        // debug!("Region: {:#?}", region);

        Aws{
            credentials,
            region
        }
    }

    /// Return cloned a AwsCredentials (used for create S3Client)
    pub(super) fn clone_credentials(&self) -> AwsCredentials {
        self.credentials.clone()
    }

    /// Return cloned a Region (used for create S3Client)
    pub(super) fn clone_region(&self) -> Region {
        self.region.clone()
    }

    /// Generate url for file download
    pub(crate) fn put_download_signed_url(
        &self,
        bucket: &str,
        path_file: &str,
        expires: u64,
    ) -> ServiceResult<String> {
        let req = rusoto_s3::GetObjectRequest {
            bucket: bucket.to_string(),
            key: path_file.to_string(),
            ..Default::default()
        };

        // debug!("GetObjectRequest: {:#?}", req);

        Ok(req.get_presigned_url(
            &self.region,
            &self.credentials,
            &PreSignedRequestOption{
                expires_in: std::time::Duration::from_secs(expires)
            }
        ))
    }

    /// Generate url for file upload
    pub(crate) fn get_upload_signed_url(
        &self,
        bucket: &str,
        path_file: &str,
        // content_sha1: &str,
        expires: u64,
    ) -> ServiceResult<String> {
        let req = rusoto_s3::PutObjectRequest{
            bucket: bucket.to_string(),
            key: path_file.to_string(),
            // content_length: Some(79_i64),
            ..Default::default()
        };

        // debug!("UploadPartRequest: {:#?}", req);

        Ok(req.get_presigned_url(
            &self.region,
            &self.credentials,
            &PreSignedRequestOption{
                expires_in: std::time::Duration::from_secs(expires)
            }
        ))
    }

    // /// Generate url for upload part
    // pub fn upload_part_signed_url(
    //     &self,
    //     bucket: &str,
    //     path_file: &str,
    //     // content_sha1: &str,
    //     expires: u64,
    // ) -> ServiceResult<String> {
    //     let req = rusoto_s3::UploadPartRequest{
    //         bucket: bucket.to_string(),
    //         key: path_file.to_string(),
    //         part_number: 1_i64,
    //         upload_id: "None".to_string(),
    //         content_length: Some(79_i64),
    //         ..Default::default()
    //     };
    //
    //     debug!("UploadPartRequest: {:#?}", req);
    //
    //     Ok(req.get_presigned_url(
    //         &self.region,
    //         &self.credentials,
    //         &PreSignedRequestOption{
    //             expires_in: std::time::Duration::from_secs(expires)
    //         }
    //     ))
    // }
}
