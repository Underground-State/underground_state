//! Database layer with PostgreSQL and Redis

pub mod pool;
pub mod redis;
pub mod repositories;

pub use pool::DatabasePool;
pub use redis::RedisPool;

use sqlx::PgPool;
use std::sync::Arc;

/// Application state containing database connections
#[derive(Clone)]
pub struct DbState {
    pub pg: PgPool,
    pub redis: RedisPool,
}

impl DbState {
    pub async fn new(database_url: &str, redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pg = DatabasePool::connect(database_url).await?;
        let redis = RedisPool::connect(redis_url).await?;

        Ok(Self { pg, redis })
    }
}
