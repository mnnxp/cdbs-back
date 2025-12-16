use crate::cli_args::Opt;
use crate::schema::*;
use chrono::NaiveDateTime;
use structopt::StructOpt;
use uuid::Uuid;

/// Proxying S3 storage URLs based on the client's domain.
///
/// Replaces the base S3 endpoint with a domain-specific proxy to optimize
/// access in a multi-regional architecture.
///
/// # Replacement Rules
///
/// - `app.cadbase.ru` → `https://s3.cadbase.ru` (Russian proxy)
/// - `app.cadbase.org` → `https://s3.cadbase.org` (international proxy)
/// - Other domains → original URL unchanged
///
/// # Example
///
/// ```
/// // When s3_endpoint = "https://s3.fr-par.scw.cloud"
/// let url = "https://s3.fr-par.scw.cloud/....jpg";
/// assert_eq!(url.proxied("app.cadbase.org"), "https://s3.cadbase.org/....jpg");
/// assert_eq!(url.proxied("unknown.com"), url); // Unchanged
/// ```
pub(crate) trait S3Proxer {
    /// Returns a URL with the S3 endpoint replaced for the specified domain.
    ///
    /// Replaces the `Opt::s3_endpoint` value in the URL string with the
    /// corresponding proxy based on the client's domain. If the domain is not
    /// found in the rules, returns the original string.
    fn proxied(&self, domain: &str) -> String;
}

impl<T: AsRef<str>> S3Proxer for T {
    fn proxied(&self, domain: &str) -> String {
        let s3_proxy = match domain {
            d if d.ends_with(".cadbase.ru") => "https://s3.cadbase.ru",
            d if d.ends_with(".cadbase.org") => "https://s3.cadbase.org",
            // d if d.ends_with("cadbase.rs") => "https://s3.fr-par.scw.cloud",
            // "localhost" => "https://s3.pl-waw.scw.cloud",
            // "127.0.0.1" => "https://s3.localhost.cloud",
            _ => return self.as_ref().to_string(),
        };
        let opt = Opt::from_args();
        self.as_ref().replace(&opt.s3_endpoint, s3_proxy)
    }
}

/// Saving an active link to the file for uses the cache browser
#[derive(Insertable, Debug)]
#[diesel(table_name = presigned_url_ref)]
pub(crate) struct InsertablePresignedUrl {
    pub(crate) file_uuid: Uuid,
    pub(crate) presigned_url: String,
    pub(crate) expiration_at: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub(crate) struct StorageAccess {
    application_key_id: String,
    application_key: String,
    pub(crate) bucket: String,
    pub(crate) region: String,
    pub(crate) endpoint: String,
}

impl StorageAccess {
    /// Gets data to access S3 from environment for generate presign-urls
    pub(crate) fn from_env() -> StorageAccess {
        let opt = Opt::from_args();

        // checking expiration date for key
        if opt.s3_access_expiration_at < chrono::Local::now().naive_local() {
            panic!("The data to access S3 is not valid.");
        }

        StorageAccess {
            application_key_id: opt.s3_application_key_id,
            application_key: opt.s3_application_key,
            bucket: opt.s3_bucket,
            region: opt.s3_region,
            endpoint: opt.s3_endpoint,
        }
    }
}

impl From<&StorageAccess> for super::s3::Aws {
    fn from(data: &StorageAccess) -> super::s3::Aws {
        let application_key_id = &data.application_key_id;
        let application_key = &data.application_key;

        super::s3::Aws::new(
            application_key_id,
            application_key,
            &data.region,
            &data.endpoint,
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct FileHeaders {
    pub(crate) content_type: Option<String>,
    pub(crate) content_length: Option<i64>,
    pub(crate) updated_at: Option<NaiveDateTime>,
}

impl From<rusoto_s3::HeadObjectOutput> for FileHeaders {
    fn from(data: rusoto_s3::HeadObjectOutput) -> Self {
        let rusoto_s3::HeadObjectOutput {
            content_length,
            content_type,
            last_modified,
            ..
        } = data;

        Self {
            content_length,
            content_type,
            updated_at: last_modified.map(|date_str| {
                NaiveDateTime::parse_from_str(date_str.as_str(), "%a, %d %b %Y %H:%M:%S GMT")
                    .unwrap()
            }),
        }
    }
}
