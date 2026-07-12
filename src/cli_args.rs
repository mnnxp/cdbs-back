use chrono::NaiveDateTime;
use structopt::StructOpt;
use uuid::Uuid;

/// GraphQl API, Diesel PostgreSQL, session authentication and JWT boilerplate server
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "cdbs-back")]
pub struct Opt {
    /// Allowed CORS origins (comma-separated)
    #[structopt(
        long,
        env = "ALLOWED_ORIGINS",
        default_value = "http://localhost:3000,http://127.0.0.1:3000"
    )]
    pub(crate) allowed_origins: String,

    /// Path to JWT private key
    #[structopt(
        long,
        env = "JWT_PRIVATE_KEY",
        default_value = "./keys/rs256-4096-private.pem"
    )]
    pub(crate) jwt_private_key: String,

    /// Path to JWT public key
    #[structopt(
        long,
        env = "JWT_PUBLIC_KEY",
        default_value = "./keys/rs256-4096-public.pem"
    )]
    pub(crate) jwt_public_key: String,

    /// Entry point of the GraphQL API (for display)
    #[structopt(
        long,
        env = "API_POINT",
        default_value = "http://127.0.0.1:3000/graphql"
    )]
    pub api_point: String,

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

    /// Root component UUID (self-referencing parent)
    #[structopt(
        long,
        env = "ROOT_COMPONENT_UUID",
        default_value = "a5953fd9-7393-4f1e-a899-06b5e159dbf1"
    )]
    pub(crate) root_component_uuid: Uuid,

    /// Root standard UUID (self-referencing parent)
    #[structopt(
        long,
        env = "ROOT_STANDARD_UUID",
        default_value = "303ec2aa-2066-42e3-93fb-de4fb9344bcb"
    )]
    pub(crate) root_standard_uuid: Uuid,

    /// Root modification UUID (self-referencing parent)
    #[structopt(
        long,
        env = "ROOT_MODIFICATION_UUID",
        default_value = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8"
    )]
    pub(crate) root_modification_uuid: Uuid,

    /// Default image UUID (placeholder)
    #[structopt(
        long,
        env = "DEFAULT_IMAGE_UUID",
        default_value = "bc1c2151-86d0-4656-9c9d-d016dd584297"
    )]
    pub(crate) default_image_uuid: Uuid,

    /// Default user UUID (anonymous)
    #[structopt(
        long,
        env = "DEFAULT_USER_UUID",
        default_value = "413a9b1c-da2d-44f9-a492-58f9448b402e"
    )]
    pub(crate) default_user_uuid: Uuid,
}
