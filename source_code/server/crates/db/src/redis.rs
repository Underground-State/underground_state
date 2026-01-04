//! Redis connection pool

use redis::aio::ConnectionManager;
use redis::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct RedisPool {
    manager: Arc<RwLock<ConnectionManager>>,
}

impl RedisPool {
    pub async fn connect(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        let manager = ConnectionManager::new(client).await?;

        Ok(Self {
            manager: Arc::new(RwLock::new(manager)),
        })
    }

    pub async fn get(&self) -> ConnectionManager {
        self.manager.read().await.clone()
    }

    /// Set a key with expiration
    pub async fn set_ex(&self, key: &str, value: &str, seconds: u64) -> Result<(), redis::RedisError> {
        let mut conn = self.get().await;
        redis::cmd("SETEX")
            .arg(key)
            .arg(seconds)
            .arg(value)
            .query_async(&mut conn)
            .await
    }

    /// Get a key
    pub async fn get_key(&self, key: &str) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.get().await;
        redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
    }

    /// Delete a key
    pub async fn del(&self, key: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.get().await;
        redis::cmd("DEL")
            .arg(key)
            .query_async(&mut conn)
            .await
    }

    /// Check if key exists
    pub async fn exists(&self, key: &str) -> Result<bool, redis::RedisError> {
        let mut conn = self.get().await;
        redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut conn)
            .await
    }
}
