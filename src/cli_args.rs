use structopt::StructOpt;
use uuid::Uuid;

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

    /// Database URL
    #[structopt(long, env = "DATABASE_URL")]
    pub database_url: String,

    /// Secret Key for Auth Cookie
    #[structopt(
        long,
        env = "AUTH_SECRET_KEY",
        default_value = "01230123012301230123012301230123"
    )]
    pub auth_secret_key: String,

    /// Use secure cookie (HTTPS),
    /// this can only be set if you have https
    #[structopt(long, env = "HTTPS_COOKIE")]
    pub secure_cookie: bool,

    /// Auth duration in hours,
    /// this is used for cookie and JWT
    #[structopt(long, env = "AUTH_DURATION_IN_HOUR", default_value = "24")]
    pub auth_duration_in_hour: u16,

    /// Backblaze
    #[structopt(long, env = "UUID_MAIN_ACCESS_B2", parse(try_from_str), default_value = "31ecc6f80c094a59a2d534b5b833e59b")]
    pub uuid_main_access_b2: Uuid, // <-- for storing the master key and token
    #[structopt(long, env = "B2_ACCOUNT_ID", default_value = "cb0e1d5d3205")]
    pub b2_account_id: String,
    #[structopt(long, env = "B2_CAPABILITIES", default_value = "listFiles,readFiles,shareFiles,writeFiles,deleteFiles")]
    pub b2_capabilities: String,
    #[structopt(long, env = "B2_BUCKET_ID", default_value = "1c8ba08e816d056d73b20015")]
    pub b2_bucket_id: String,
    #[structopt(long, env = "B2_BUCKET", default_value = "cdbs-test")]
    pub b2_bucket: String,
    #[structopt(long, env = "S3_REGION", default_value = "s3.us-west-002")]
    pub s3_region: String,
    #[structopt(long, env = "S3_ENDPOINT", default_value = "backblazeb2.com")]
    pub s3_endpoint: String,
}
