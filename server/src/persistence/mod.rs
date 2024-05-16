use diesel::r2d2::{ConnectionManager, Pool as R2D2Pool};
use diesel::PgConnection;
use std::env;

pub type Connection = PgConnection;
pub type ConnectionPool = R2D2Pool<ConnectionManager<Connection>>;

#[derive(thiserror::Error, Debug)]
pub enum PoolCreationError {
    #[error("Could not build a connection pool: {0:?}")]
    CouldNotBuild(String),
    #[error("Could not read $DATABASE_URL: {0:?}")]
    NoEnv(String),
}

#[tracing::instrument]
pub fn create_connection_pool() -> Result<ConnectionPool, PoolCreationError> {
    let url = env::var("DATABASE_URL").map_err(|err| PoolCreationError::NoEnv(err.to_string()))?;
    tracing::debug!(message = "Creating a PoolBuilder", ?url);
    let manager = ConnectionManager::<Connection>::new(url);
    ConnectionPool::builder()
        .test_on_check_out(true)
        .build(manager)
        .map_err(|err| PoolCreationError::CouldNotBuild(err.to_string()))
}
