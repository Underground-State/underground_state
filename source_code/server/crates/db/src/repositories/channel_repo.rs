//! Channel repository

use sqlx::PgPool;
use ug_core::{ChannelId, GuildId, ChannelType};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ChannelRow {
    pub id: i64,
    pub guild_id: Option<i64>,
    pub channel_type: i16,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub position: i16,
    pub parent_id: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct ChannelRepository;

impl ChannelRepository {
    pub async fn find_by_id(pool: &PgPool, id: ChannelId) -> Result<Option<ChannelRow>, sqlx::Error> {
        sqlx::query_as::<_, ChannelRow>(
            "SELECT id, guild_id, channel_type, name, topic, position, parent_id, created_at
             FROM channels WHERE id = $1"
        )
        .bind(id.as_i64())
        .fetch_optional(pool)
        .await
    }

    pub async fn list_by_guild(pool: &PgPool, guild_id: GuildId) -> Result<Vec<ChannelRow>, sqlx::Error> {
        sqlx::query_as::<_, ChannelRow>(
            "SELECT id, guild_id, channel_type, name, topic, position, parent_id, created_at
             FROM channels WHERE guild_id = $1
             ORDER BY position"
        )
        .bind(guild_id.as_i64())
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        id: i64,
        guild_id: GuildId,
        channel_type: ChannelType,
        name: &str,
    ) -> Result<ChannelRow, sqlx::Error> {
        let next_position: (i16,) = sqlx::query_as(
            "SELECT COALESCE(MAX(position), 0) + 1 FROM channels WHERE guild_id = $1"
        )
        .bind(guild_id.as_i64())
        .fetch_one(pool)
        .await?;

        sqlx::query_as::<_, ChannelRow>(
            "INSERT INTO channels (id, guild_id, channel_type, name, position, created_at)
             VALUES ($1, $2, $3, $4, $5, NOW())
             RETURNING id, guild_id, channel_type, name, topic, position, parent_id, created_at"
        )
        .bind(id)
        .bind(guild_id.as_i64())
        .bind(channel_type as i16)
        .bind(name)
        .bind(next_position.0)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: ChannelId) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM channels WHERE id = $1")
            .bind(id.as_i64())
            .execute(pool)
            .await?;
        Ok(())
    }
}
