use structopt::StructOpt;
use chrono::NaiveDateTime;

/// GraphQl API, Diesel PostgreSQL, session authentication and JWT boilerplate server
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "cdbs-back")]
pub struct Opt {
    /// Port to listen to
    #[structopt(short, long, env = "PORT", default_value = "3000")]
    pub port: u16,

    /// Domain
    #[structopt(long, env = "DOMAIN", default_value = "localhost")]
    pub domain: String,

    /// Database host
    #[structopt(long, env = "POSTGRES_HOST")]
    pub(crate) postgres_host: String,

    /// Database user
    #[structopt(long, env = "POSTGRES_USER")]
    pub(crate) postgres_user: String,

    /// Database password
    #[structopt(long, env = "POSTGRES_PASSWORD")]
    pub(crate) postgres_password: String,

    /// Database name database
    #[structopt(long, env = "POSTGRES_DB")]
    pub(crate) postgres_db: String,

    /// Auth duration in hours,
    /// this is used for cookie and JWT
    #[structopt(long, env = "AUTH_DURATION_IN_HOUR", default_value = "24")]
    pub(crate) auth_duration_in_hour: u16,

    /// Upload/download URL expiration date for S3
    #[structopt(long, env = "S3_EXP_PRESIGNED_URL", default_value = "800")]
    pub(crate) s3_expiration_presigned_url: u64,

    /// Application key id for S3
    #[structopt(long, env = "S3_APPLICATION_KEY_ID")]
    pub(crate) s3_application_key_id: String,

    /// Application key for S3
    #[structopt(long, env = "S3_APPLICATION_KEY")]
    pub(crate) s3_application_key: String,

    /// Key expiration date for S3
    #[structopt(long, env = "S3_ACCESS_EXPIRATION_AT")]
    pub(crate) s3_access_expiration_at: NaiveDateTime,

    /// Bucket name from S3
    #[structopt(long, env = "S3_BUCKET")]
    pub(crate) s3_bucket: String,

    /// Server location S3
    #[structopt(long, env = "S3_REGION")]
    pub(crate) s3_region: String,

    /// Url to endpoint S3
    #[structopt(long, env = "S3_ENDPOINT")]
    pub(crate) s3_endpoint: String,
}
