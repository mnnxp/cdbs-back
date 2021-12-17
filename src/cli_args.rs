use structopt::StructOpt;

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
    pub(crate) database_url: String,

    /// Auth duration in hours,
    /// this is used for cookie and JWT
    #[structopt(long, env = "AUTH_DURATION_IN_HOUR", default_value = "24")]
    pub(crate) auth_duration_in_hour: u16,

    /// Expiration upload/download URL
    #[structopt(long, env = "EXP_PRESIGNED_URL")]
    pub(crate) expiration_presigned_url: u64,
}
