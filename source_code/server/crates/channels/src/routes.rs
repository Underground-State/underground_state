//! Channel routes

use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    routing::{get, delete},
    Json, Router,
};
use serde::Serialize;

use ug_auth::Claims;
use ug_core::ChannelId;
use ug_db::{repositories::ChannelRepository, DbState};

#[derive(Clone)]
pub struct ChannelsState {
    pub db: DbState,
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

pub fn channels_router() -> Router<ChannelsState> {
    Router::new()
        .route("/:channel_id", get(get_channel).delete(delete_channel))
}

/// Get channel by ID
async fn get_channel(
    State(state): State<ChannelsState>,
    Extension(_claims): Extension<Claims>,
    Path(channel_id): Path<i64>,
) -> Result<Json<ChannelResponse>, (StatusCode, String)> {
    let channel = ChannelRepository::find_by_id(&state.db.pg, ChannelId::new(channel_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Channel not found".to_string()))?;

    Ok(Json(ChannelResponse {
        id: channel.id.to_string(),
        guild_id: channel.guild_id.map(|id| id.to_string()),
        channel_type: channel.channel_type,
        name: channel.name,
        topic: channel.topic,
        position: channel.position,
    }))
}

/// Delete channel
async fn delete_channel(
    State(state): State<ChannelsState>,
    Extension(_claims): Extension<Claims>,
    Path(channel_id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    // TODO: Check permissions

    ChannelRepository::delete(&state.db.pg, ChannelId::new(channel_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
