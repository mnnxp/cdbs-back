use crate::errors::ServiceResult;
use crate::cli_args;
use rusoto_core::{
    Region,
    credential::{
        AwsCredentials,
        EnvironmentProvider,
        ProvideAwsCredentials
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
    /// Create new access to S3
    pub async fn new(
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

        std::env::set_var("S3_CDBS_ACCESS_KEY_ID", access_key_id);
        std::env::set_var("S3_CDBS_SECRET_ACCESS_KEY", secret_access_key);
        std::env::set_var("S3_CDBS_SESSION_TOKEN", session_token);

        let credentials = EnvironmentProvider::with_prefix("S3_CDBS").credentials().await.unwrap();
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
            &PreSignedRequestOption {
                expires_in: std::time::Duration::from_secs(expires)
            },
        ))
    }

    /// Generate url for file upload
    pub fn _put_upload_signed_url(
        &self,
        bucket: &str,
        path_file: &str,
        expires: i64,
    ) {
        use super::ext::rusoto_s3::PostPolicy;

        // let random_key = path_file.to_string();
        let key = path_file.to_string();
        let bucket = bucket.to_string();
        let region = &self.region;
        let credentials = &self.credentials;
        let expiration_date = chrono::Utc::now() + chrono::Duration::seconds(expires);
        let content_length = 33_u64;
        let (url, form_data) = PostPolicy::default()
            .set_content_type("bytes")
            .set_bucket_name(&bucket)
            .set_region(region)
            .set_access_key_id(credentials.aws_access_key_id())
            .set_secret_access_key(credentials.aws_secret_access_key())
            .set_key(&key)
            .set_expiration(&expiration_date)
            .set_content_length_range(content_length, content_length + 1)
            .build_form_data().unwrap();

        debug!("PostPolicy: url {:#?}, form_data {:#?}", url, form_data);
    }
}
