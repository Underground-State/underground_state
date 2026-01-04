//! User repository

use sqlx::PgPool;
use ug_core::{UserId, KycStatus};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRow {
    pub id: i64,
    pub wallet_address: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_hash: Option<String>,
    pub kyc_status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(pool: &PgPool, id: UserId) -> Result<Option<UserRow>, sqlx::Error> {
        sqlx::query_as::<_, UserRow>(
            "SELECT id, wallet_address, username, display_name, avatar_hash, kyc_status, created_at, updated_at
             FROM users WHERE id = $1"
        )
        .bind(id.as_i64())
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_wallet(pool: &PgPool, wallet: &str) -> Result<Option<UserRow>, sqlx::Error> {
        sqlx::query_as::<_, UserRow>(
            "SELECT id, wallet_address, username, display_name, avatar_hash, kyc_status, created_at, updated_at
             FROM users WHERE wallet_address = $1"
        )
        .bind(wallet.to_lowercase())
        .fetch_optional(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        id: i64,
        wallet_address: &str,
        username: &str,
    ) -> Result<UserRow, sqlx::Error> {
        sqlx::query_as::<_, UserRow>(
            "INSERT INTO users (id, wallet_address, username, kyc_status, created_at, updated_at)
             VALUES ($1, $2, $3, 'none', NOW(), NOW())
             RETURNING id, wallet_address, username, display_name, avatar_hash, kyc_status, created_at, updated_at"
        )
        .bind(id)
        .bind(wallet_address.to_lowercase())
        .bind(username)
        .fetch_one(pool)
        .await
    }

    pub async fn update_username(
        pool: &PgPool,
        id: UserId,
        username: &str,
    ) -> Result<UserRow, sqlx::Error> {
        sqlx::query_as::<_, UserRow>(
            "UPDATE users SET username = $2, updated_at = NOW() WHERE id = $1
             RETURNING id, wallet_address, username, display_name, avatar_hash, kyc_status, created_at, updated_at"
        )
        .bind(id.as_i64())
        .bind(username)
        .fetch_one(pool)
        .await
    }

    pub async fn update_kyc_status(
        pool: &PgPool,
        id: UserId,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET kyc_status = $2, updated_at = NOW() WHERE id = $1")
            .bind(id.as_i64())
            .bind(status)
            .execute(pool)
            .await?;
        Ok(())
    }
}
