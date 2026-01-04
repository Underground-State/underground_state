//! Message repository

use sqlx::PgPool;
use ug_core::{MessageId, ChannelId, UserId, Pagination};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MessageRow {
    pub id: i64,
    pub channel_id: i64,
    pub author_id: i64,
    pub content: String,
    pub message_type: i16,
    pub reply_to_id: Option<i64>,
    pub edited_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_deleted: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct MessageRepository;

impl MessageRepository {
    pub async fn find_by_id(pool: &PgPool, id: MessageId) -> Result<Option<MessageRow>, sqlx::Error> {
        sqlx::query_as::<_, MessageRow>(
            "SELECT id, channel_id, author_id, content, message_type, reply_to_id, edited_at, is_deleted, created_at
             FROM messages WHERE id = $1 AND is_deleted = false"
        )
        .bind(id.as_i64())
        .fetch_optional(pool)
        .await
    }

    pub async fn list_by_channel(
        pool: &PgPool,
        channel_id: ChannelId,
        pagination: &Pagination,
    ) -> Result<Vec<MessageRow>, sqlx::Error> {
        let limit = pagination.limit.min(100) as i64;

        if let Some(before) = pagination.before {
            sqlx::query_as::<_, MessageRow>(
                "SELECT id, channel_id, author_id, content, message_type, reply_to_id, edited_at, is_deleted, created_at
                 FROM messages
                 WHERE channel_id = $1 AND id < $2 AND is_deleted = false
                 ORDER BY id DESC
                 LIMIT $3"
            )
            .bind(channel_id.as_i64())
            .bind(before)
            .bind(limit)
            .fetch_all(pool)
            .await
        } else if let Some(after) = pagination.after {
            sqlx::query_as::<_, MessageRow>(
                "SELECT id, channel_id, author_id, content, message_type, reply_to_id, edited_at, is_deleted, created_at
                 FROM messages
                 WHERE channel_id = $1 AND id > $2 AND is_deleted = false
                 ORDER BY id ASC
                 LIMIT $3"
            )
            .bind(channel_id.as_i64())
            .bind(after)
            .bind(limit)
            .fetch_all(pool)
            .await
        } else {
            sqlx::query_as::<_, MessageRow>(
                "SELECT id, channel_id, author_id, content, message_type, reply_to_id, edited_at, is_deleted, created_at
                 FROM messages
                 WHERE channel_id = $1 AND is_deleted = false
                 ORDER BY id DESC
                 LIMIT $2"
            )
            .bind(channel_id.as_i64())
            .bind(limit)
            .fetch_all(pool)
            .await
        }
    }

    pub async fn create(
        pool: &PgPool,
        id: i64,
        channel_id: ChannelId,
        author_id: UserId,
        content: &str,
        reply_to_id: Option<MessageId>,
    ) -> Result<MessageRow, sqlx::Error> {
        sqlx::query_as::<_, MessageRow>(
            "INSERT INTO messages (id, channel_id, author_id, content, message_type, reply_to_id, is_deleted, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, false, NOW())
             RETURNING id, channel_id, author_id, content, message_type, reply_to_id, edited_at, is_deleted, created_at"
        )
        .bind(id)
        .bind(channel_id.as_i64())
        .bind(author_id.as_i64())
        .bind(content)
        .bind(if reply_to_id.is_some() { 1i16 } else { 0i16 })
        .bind(reply_to_id.map(|id| id.as_i64()))
        .fetch_one(pool)
        .await
    }

    pub async fn update_content(
        pool: &PgPool,
        id: MessageId,
        content: &str,
    ) -> Result<MessageRow, sqlx::Error> {
        sqlx::query_as::<_, MessageRow>(
            "UPDATE messages SET content = $2, edited_at = NOW() WHERE id = $1
             RETURNING id, channel_id, author_id, content, message_type, reply_to_id, edited_at, is_deleted, created_at"
        )
        .bind(id.as_i64())
        .bind(content)
        .fetch_one(pool)
        .await
    }

    pub async fn soft_delete(pool: &PgPool, id: MessageId) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE messages SET is_deleted = true WHERE id = $1")
            .bind(id.as_i64())
            .execute(pool)
            .await?;
        Ok(())
    }
}
