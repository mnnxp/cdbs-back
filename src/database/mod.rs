pub mod pool;

use crate::errors::ServiceError;
use async_graphql::Context;
use diesel::r2d2::PoolError;

type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;

pub(crate) fn db_connection(pool: &Pool) -> Result<PooledConnection, ServiceError> {
    let conn = pool.get().map_err(|_| ServiceError::UnableToConnectToDb)?;
    Ok(conn)
}

pub(crate) fn get_conn<'a>(context: &Context<'a>) -> Result<PooledConnection, ServiceError> {
    let conn = context
        .data::<Pool>().expect("Can't get pool")
        .get().map_err(|_| ServiceError::UnableToConnectToDb)?;
    Ok(conn)
}
