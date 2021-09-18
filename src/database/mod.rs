pub(crate) mod pool;

use crate::errors::ServiceError;
use async_graphql::Context;
use diesel::r2d2::PoolError;

type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::PgConnection>;
pub(crate) type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub(crate) type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;
pub(crate) type PgPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

pub(crate) fn db_connection(pool: &Pool) -> Result<PooledConnection, ServiceError> {
    let conn = pool.get().map_err(|_| ServiceError::UnableToConnectToDb)?;
    Ok(conn)
}

pub(crate) fn get_conn(cxt: &Context) -> Result<PooledConnection, ServiceError> {
    let conn = cxt
        .data::<Pool>().expect("Can't get pool")
        .get().map_err(|_| ServiceError::UnableToConnectToDb)?;
    Ok(conn)
}

pub(crate) fn get_pool(cxt: &Context) -> Result<PgPool, ServiceError> {
    let pool = cxt
        .data::<Pool>().expect("Can't get pool")
        .clone();
    Ok(pool)
}
