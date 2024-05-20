use diesel::r2d2::{ConnectionManager, Pool as R2D2Pool};
use diesel::PgConnection;
use std::env;

pub type Connection = PgConnection;
pub type ConnectionPool = R2D2Pool<ConnectionManager<Connection>>;

#[tracing::instrument]
pub fn create_persistence_pool() -> ConnectionPool {
    let url = env::var("DATABASE_URL").expect("Could not read $DATABASE_URL");
    tracing::debug!(message = "Creating a PostgreSQL connection pool", ?url);
    let manager = ConnectionManager::<Connection>::new(url);
    ConnectionPool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build a connection pool")
}
