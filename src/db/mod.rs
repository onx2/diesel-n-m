use anyhow::Result;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenvy::dotenv;
use std::env;

/// This is used as a wrapper for a connection pool and provides the application's
/// default settings for pools and test connections.
///
/// It also provides some helper methods for getting a connection pool and connection
/// from a pool.
#[derive(Debug, Clone)]
pub struct Db {
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl Db {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        Self {
            connection_pool: Pool::builder()
                .max_size(15)
                .build(manager)
                .expect("Failed to get pool"),
        }
    }

    /// Retrieves a connection from the Postgres connection pool.
    ///
    /// Errors produced from this method are automatically logged at `error` level
    pub fn connect(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        match self.connection_pool.clone().get() {
            Ok(conn) => Ok(conn),
            Err(err) => Err(err.into()),
        }
    }
}
