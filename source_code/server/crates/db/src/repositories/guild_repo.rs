//! Guild repository

use sqlx::PgPool;
use ug_core::{GuildId, UserId};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GuildRow {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub icon_hash: Option<String>,
    pub description: Option<String>,
    pub member_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GuildMemberRow {
    pub guild_id: i64,
    pub user_id: i64,
    pub nickname: Option<String>,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

pub struct GuildRepository;

impl GuildRepository {
    pub async fn find_by_id(pool: &PgPool, id: GuildId) -> Result<Option<GuildRow>, sqlx::Error> {
        sqlx::query_as::<_, GuildRow>(
            "SELECT id, owner_id, name, icon_hash, description, member_count, created_at, updated_at
             FROM guilds WHERE id = $1"
        )
        .bind(id.as_i64())
        .fetch_optional(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        id: i64,
        owner_id: UserId,
        name: &str,
    ) -> Result<GuildRow, sqlx::Error> {
        sqlx::query_as::<_, GuildRow>(
            "INSERT INTO guilds (id, owner_id, name, member_count, created_at, updated_at)
             VALUES ($1, $2, $3, 1, NOW(), NOW())
             RETURNING id, owner_id, name, icon_hash, description, member_count, created_at, updated_at"
        )
        .bind(id)
        .bind(owner_id.as_i64())
        .bind(name)
        .fetch_one(pool)
        .await
    }

    pub async fn list_by_user(pool: &PgPool, user_id: UserId) -> Result<Vec<GuildRow>, sqlx::Error> {
        sqlx::query_as::<_, GuildRow>(
            "SELECT g.id, g.owner_id, g.name, g.icon_hash, g.description, g.member_count, g.created_at, g.updated_at
             FROM guilds g
             INNER JOIN guild_members gm ON g.id = gm.guild_id
             WHERE gm.user_id = $1
             ORDER BY g.name"
        )
        .bind(user_id.as_i64())
        .fetch_all(pool)
        .await
    }

    pub async fn add_member(
        pool: &PgPool,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query(
            "INSERT INTO guild_members (guild_id, user_id, joined_at)
             VALUES ($1, $2, NOW())
             ON CONFLICT (guild_id, user_id) DO NOTHING"
        )
        .bind(guild_id.as_i64())
        .bind(user_id.as_i64())
        .execute(&mut *tx)
        .await?;

        sqlx::query("UPDATE guilds SET member_count = member_count + 1 WHERE id = $1")
            .bind(guild_id.as_i64())
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn remove_member(
        pool: &PgPool,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query("DELETE FROM guild_members WHERE guild_id = $1 AND user_id = $2")
            .bind(guild_id.as_i64())
            .bind(user_id.as_i64())
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE guilds SET member_count = member_count - 1 WHERE id = $1")
            .bind(guild_id.as_i64())
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn is_member(
        pool: &PgPool,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<bool, sqlx::Error> {
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM guild_members WHERE guild_id = $1 AND user_id = $2"
        )
        .bind(guild_id.as_i64())
        .bind(user_id.as_i64())
        .fetch_optional(pool)
        .await?;

        Ok(result.is_some())
    }
}
