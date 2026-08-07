use crate::cli_args::Opt;
use crate::storage::s3::Aws;
use chrono::Local;
use std::sync::OnceLock;
use uuid::Uuid;

/// Global storage for application configuration
static CONFIG: OnceLock<Opt> = OnceLock::new();

/// Initializes the global configuration
pub(crate) fn init_config(opt: Opt) -> Result<(), &'static str> {
    if opt.s3_access_expiration_at < Local::now().naive_local() {
        return Err("The data to access S3 is expired.");
    }
    if CONFIG.set(opt).is_err() {
        log::warn!("Configuration has already been initialized!");
    }
    Ok(())
}

/// Returns a static reference to the global configuration
fn get_config() -> &'static Opt {
    CONFIG.get().expect("Config is not initialized")
}

/// Returns the GraphQL API endpoint URL
pub(crate) fn api_point() -> String {
    get_config().api_point.clone()
}

/// Returns the token expiration duration in hours
pub(crate) fn auth_duration_in_hour() -> u16 {
    get_config().auth_duration_in_hour
}

/// Returns the formatted PostgreSQL database connection string
pub(crate) fn database_url() -> String {
    let opt = get_config();
    format!(
        "postgres://{user}:{password}@{host}/{database}",
        user = opt.postgres_user,
        password = opt.postgres_password,
        host = opt.postgres_host,
        database = opt.postgres_db
    )
}

/// Returns the configured S3 endpoint URL
pub(crate) fn s3_endpoint() -> &'static str {
    &get_config().s3_endpoint
}

/// Returns the configured S3 bucket name
pub fn s3_bucket() -> &'static str {
    &get_config().s3_bucket
}

/// Returns the S3 presigned URL expiration time in seconds
pub(crate) fn s3_expiration_presigned_url() -> u64 {
    get_config().s3_expiration_presigned_url
}

/// Creates an Aws client instance
pub(crate) fn aws_client() -> Aws {
    let opt = get_config();
    Aws::new(
        &opt.s3_application_key_id,
        &opt.s3_application_key,
        &opt.s3_region,
        &opt.s3_endpoint,
    )
}

/// Root component UUID (self-referencing parent)
pub(crate) fn root_component_uuid() -> Uuid {
    get_config().root_component_uuid
}

/// Root standard UUID (self-referencing parent)
pub(crate) fn root_standard_uuid() -> Uuid {
    get_config().root_standard_uuid
}

/// Root modification UUID (self-referencing parent)
pub(crate) fn root_modification_uuid() -> Uuid {
    get_config().root_modification_uuid
}

/// Root discussion comment UUID (self-referencing parent)
pub(crate) fn root_discussion_comment_uuid() -> Uuid {
    get_config().root_discussion_comment_uuid
}

/// Default image UUID (placeholder)
pub(crate) fn default_image_uuid() -> Uuid {
    get_config().default_image_uuid
}

/// Default user UUID (anonymous)
pub(crate) fn default_user_uuid() -> Uuid {
    get_config().default_user_uuid
}
