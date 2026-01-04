//! PostgreSQL connection pool

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

pub struct DatabasePool;

impl DatabasePool {
    pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(50)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(600))
            .connect(database_url)
            .await
    }
}
