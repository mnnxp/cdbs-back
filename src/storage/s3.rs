use crate::errors::ServiceResult;

#[derive(Clone)]
pub struct Aws {
    credentials: String,
    region: String
}

impl Aws {
    /// Create new access to S3
    pub async fn new(
        access_key_id: &str,
        secret_access_key: &str,
        aws_region: &str,
    ) -> ServiceResult<Aws> {

        std::env::set_var("AWS_ACCESS_KEY_ID", access_key_id);
        std::env::set_var("AWS_SECRET_ACCESS_KEY", secret_access_key);
        std::env::set_var("AWS_REGION", aws_region);

        Ok(Aws{
            credentials: String::new(),
            region: String::new()
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
