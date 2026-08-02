use super::{ConnectionManager, Pool, PoolError};
use crate::config::database_url;

fn init_pool(database_url: &str) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::new(database_url);
    Pool::builder().max_size(5).build(manager)
}

pub(crate) fn establish_connection() -> Pool {
    // postgres://{user}:{password}@{host}/{database}
    let database_url = database_url();
    init_pool(&database_url).expect("Failed to create pool")
}
