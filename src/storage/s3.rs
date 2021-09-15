use crate::errors::ServiceResult;
use crate::cli_args;
use rusoto_signature::{
    Region,
    credential::{
        AwsCredentials,
        // EnvironmentProvider,
        // ProvideAwsCredentials
    }
};
use rusoto_s3::util::{
    PreSignedRequest,
    PreSignedRequestOption,
};

#[derive(Clone)]
pub struct Aws {
    credentials: AwsCredentials,
    region: Region
}

impl Aws {
    pub fn new(
        access_key_id: &str,
        secret_access_key: &str,
        session_token: &str,
    ) -> ServiceResult<Aws> {
        // Gets enviroment variables from `.env`
        dotenv::dotenv().ok();

        // Sets options to enviroment variables
        let opt = {
            use structopt::StructOpt;
            cli_args::Opt::from_args()
        };

        let credentials = AwsCredentials::new(
            access_key_id,
            secret_access_key,
            Some(session_token.to_string()),
            None
        );

        debug!("Credentials: {:#?}", credentials);

        let region = Region::Custom {
            name: opt.s3_region,
            endpoint: opt.s3_endpoint,
        };

        debug!("Region: {:#?}", region);

        Ok(Aws{
            credentials,
            region
        })
    }

    /// Generate url for file download
    pub fn put_download_signed_url(
        &self,
        bucket: &str,
        path_file: &str,
        expires: u64,
    ) -> ServiceResult<String> {
        let req = rusoto_s3::PutObjectRequest {
            bucket: bucket.to_string(),
            key: path_file.to_string(),
            content_length: Some(33_i64),
            ..Default::default()
        };

        debug!("PutObjectRequest: {:#?}", req);

        Ok(req.get_presigned_url(
            &self.region,
            &self.credentials,
            &PreSignedRequestOption{
                expires_in: std::time::Duration::from_secs(expires)
            }
        ))
    }

    /// Generate url for file upload
    pub fn get_upload_signed_url(
        &self,
        bucket: &str,
        path_file: &str,
        // content_sha1: &str,
        expires: u64,
    ) -> ServiceResult<String> {
        let req = rusoto_s3::UploadPartRequest{
            bucket: bucket.to_owned(),
            key: path_file.to_owned(),
            content_length: Some(33_i64),
            ..Default::default()
        };

        debug!("PutObjectRequest: {:#?}", req);

        Ok(req.get_presigned_url(
            &self.region,
            &self.credentials,
            &PreSignedRequestOption{
                expires_in: std::time::Duration::from_secs(expires)
            }
        ))
    }
}
