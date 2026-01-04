//! Guild routes

use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use ug_auth::Claims;
use ug_common::snowflake::generate_id;
use ug_core::{GuildId, ChannelType};
use ug_db::{
    repositories::{GuildRepository, ChannelRepository},
    DbState,
};

#[derive(Clone)]
pub struct GuildsState {
    pub db: DbState,
}

#[derive(Debug, Serialize)]
pub struct GuildResponse {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub icon_hash: Option<String>,
    pub description: Option<String>,
    pub member_count: i32,
}

#[derive(Debug, Serialize)]
pub struct ChannelResponse {
    pub id: String,
    pub guild_id: Option<String>,
    pub channel_type: i16,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub position: i16,
}

#[derive(Debug, Deserialize)]
pub struct CreateGuildRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    #[serde(default)]
    pub channel_type: Option<i16>,
}

pub fn guilds_router() -> Router<GuildsState> {
    Router::new()
        .route("/", get(list_guilds).post(create_guild))
        .route("/:guild_id", get(get_guild).delete(delete_guild))
        .route("/:guild_id/channels", get(list_channels).post(create_channel))
        .route("/:guild_id/members", get(list_members))
        .route("/:guild_id/members/@me", delete(leave_guild))
}

/// List user's guilds
async fn list_guilds(
    State(state): State<GuildsState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<GuildResponse>>, (StatusCode, String)> {
    let guilds = GuildRepository::list_by_user(&state.db.pg, claims.user_id())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(
        guilds
            .into_iter()
            .map(|g| GuildResponse {
                id: g.id.to_string(),
                owner_id: g.owner_id.to_string(),
                name: g.name,
                icon_hash: g.icon_hash,
                description: g.description,
                member_count: g.member_count,
            })
            .collect(),
    ))
}

/// Create a new guild
async fn create_guild(
    State(state): State<GuildsState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateGuildRequest>,
) -> Result<Json<GuildResponse>, (StatusCode, String)> {
    let guild_id = generate_id();

    // Create guild
    let guild = GuildRepository::create(&state.db.pg, guild_id, claims.user_id(), &req.name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Add owner as member
    GuildRepository::add_member(&state.db.pg, GuildId::new(guild_id), claims.user_id())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create default channels
    let general_id = generate_id();
    ChannelRepository::create(&state.db.pg, general_id, GuildId::new(guild_id), ChannelType::Text, "general")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let voice_id = generate_id();
    ChannelRepository::create(&state.db.pg, voice_id, GuildId::new(guild_id), ChannelType::Voice, "General")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(GuildResponse {
        id: guild.id.to_string(),
        owner_id: guild.owner_id.to_string(),
        name: guild.name,
        icon_hash: guild.icon_hash,
        description: guild.description,
        member_count: guild.member_count,
    }))
}

/// Get guild by ID
async fn get_guild(
    State(state): State<GuildsState>,
    Extension(claims): Extension<Claims>,
    Path(guild_id): Path<i64>,
) -> Result<Json<GuildResponse>, (StatusCode, String)> {
    // Check membership
    let is_member = GuildRepository::is_member(&state.db.pg, GuildId::new(guild_id), claims.user_id())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !is_member {
        return Err((StatusCode::FORBIDDEN, "Not a member".to_string()));
    }

    let guild = GuildRepository::find_by_id(&state.db.pg, GuildId::new(guild_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guild not found".to_string()))?;

    Ok(Json(GuildResponse {
        id: guild.id.to_string(),
        owner_id: guild.owner_id.to_string(),
        name: guild.name,
        icon_hash: guild.icon_hash,
        description: guild.description,
        member_count: guild.member_count,
    }))
}

/// Delete guild (owner only)
async fn delete_guild(
    State(state): State<GuildsState>,
    Extension(claims): Extension<Claims>,
    Path(guild_id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    let guild = GuildRepository::find_by_id(&state.db.pg, GuildId::new(guild_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guild not found".to_string()))?;

    if guild.owner_id != claims.user_id().as_i64() {
        return Err((StatusCode::FORBIDDEN, "Only owner can delete guild".to_string()));
    }

    // Delete guild (cascade will handle members, channels, messages)
    sqlx::query("DELETE FROM guilds WHERE id = $1")
        .bind(guild_id)
        .execute(&state.db.pg)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

/// List guild channels
async fn list_channels(
    State(state): State<GuildsState>,
    Extension(claims): Extension<Claims>,
    Path(guild_id): Path<i64>,
) -> Result<Json<Vec<ChannelResponse>>, (StatusCode, String)> {
    // Check membership
    let is_member = GuildRepository::is_member(&state.db.pg, GuildId::new(guild_id), claims.user_id())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !is_member {
        return Err((StatusCode::FORBIDDEN, "Not a member".to_string()));
    }

    let channels = ChannelRepository::list_by_guild(&state.db.pg, GuildId::new(guild_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(
        channels
            .into_iter()
            .map(|c| ChannelResponse {
                id: c.id.to_string(),
                guild_id: c.guild_id.map(|id| id.to_string()),
                channel_type: c.channel_type,
                name: c.name,
                topic: c.topic,
                position: c.position,
            })
            .collect(),
    ))
}

/// Create channel
async fn create_channel(
    State(state): State<GuildsState>,
    Extension(claims): Extension<Claims>,
    Path(guild_id): Path<i64>,
    Json(req): Json<CreateChannelRequest>,
) -> Result<Json<ChannelResponse>, (StatusCode, String)> {
    // Check if user is owner or has manage channels permission
    let guild = GuildRepository::find_by_id(&state.db.pg, GuildId::new(guild_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guild not found".to_string()))?;

    if guild.owner_id != claims.user_id().as_i64() {
        return Err((StatusCode::FORBIDDEN, "Permission denied".to_string()));
    }

    let channel_type = ChannelType::from(req.channel_type.unwrap_or(0));
    let channel_id = generate_id();

    let channel = ChannelRepository::create(
        &state.db.pg,
        channel_id,
        GuildId::new(guild_id),
        channel_type,
        &req.name,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ChannelResponse {
        id: channel.id.to_string(),
        guild_id: channel.guild_id.map(|id| id.to_string()),
        channel_type: channel.channel_type,
        name: channel.name,
        topic: channel.topic,
        position: channel.position,
    }))
}

/// List guild members
async fn list_members(
    State(_state): State<GuildsState>,
    Extension(_claims): Extension<Claims>,
    Path(_guild_id): Path<i64>,
) -> Result<Json<Vec<()>>, (StatusCode, String)> {
    // TODO: Implement member listing
    Ok(Json(vec![]))
}

/// Leave guild
async fn leave_guild(
    State(state): State<GuildsState>,
    Extension(claims): Extension<Claims>,
    Path(guild_id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Check if owner (can't leave own guild)
    let guild = GuildRepository::find_by_id(&state.db.pg, GuildId::new(guild_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guild not found".to_string()))?;

    if guild.owner_id == claims.user_id().as_i64() {
        return Err((StatusCode::BAD_REQUEST, "Owner cannot leave guild".to_string()));
    }

    GuildRepository::remove_member(&state.db.pg, GuildId::new(guild_id), claims.user_id())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
