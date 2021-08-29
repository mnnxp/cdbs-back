use super::{ConnectionManager, Pool, PoolError};

fn init_pool(database_url: &str) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::new(database_url);
    Pool::builder()
        .max_size(5)
        .build(manager)
}

pub(crate) fn establish_connection(opt: crate::cli_args::Opt) -> Pool {
    init_pool(&opt.database_url).expect("Failed to create pool")
}
