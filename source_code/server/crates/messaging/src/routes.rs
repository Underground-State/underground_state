//! Message routes

use axum::{
    extract::{Path, Query, State, Extension},
    http::StatusCode,
    routing::{get, post, patch, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use ug_auth::Claims;
use ug_common::snowflake::generate_id;
use ug_core::{ChannelId, MessageId, Pagination};
use ug_db::{repositories::MessageRepository, DbState};

use crate::Gateway;
use std::sync::Arc;

#[derive(Clone)]
pub struct MessagingState {
    pub db: DbState,
    pub gateway: Arc<Gateway>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub id: String,
    pub channel_id: String,
    pub author_id: String,
    pub content: String,
    pub message_type: i16,
    pub reply_to_id: Option<String>,
    pub edited_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
    #[serde(default)]
    pub reply_to_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMessageRequest {
    pub content: String,
}

pub fn messaging_router() -> Router<MessagingState> {
    Router::new()
        .route(
            "/channels/:channel_id/messages",
            get(list_messages).post(create_message),
        )
        .route(
            "/channels/:channel_id/messages/:message_id",
            get(get_message).patch(update_message).delete(delete_message),
        )
}

/// List messages in channel
async fn list_messages(
    State(state): State<MessagingState>,
    Extension(_claims): Extension<Claims>,
    Path(channel_id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<MessageResponse>>, (StatusCode, String)> {
    let messages = MessageRepository::list_by_channel(
        &state.db.pg,
        ChannelId::new(channel_id),
        &pagination,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(
        messages
            .into_iter()
            .map(|m| MessageResponse {
                id: m.id.to_string(),
                channel_id: m.channel_id.to_string(),
                author_id: m.author_id.to_string(),
                content: m.content,
                message_type: m.message_type,
                reply_to_id: m.reply_to_id.map(|id| id.to_string()),
                edited_at: m.edited_at.map(|t| t.to_rfc3339()),
                created_at: m.created_at.to_rfc3339(),
            })
            .collect(),
    ))
}

/// Create message
async fn create_message(
    State(state): State<MessagingState>,
    Extension(claims): Extension<Claims>,
    Path(channel_id): Path<i64>,
    Json(req): Json<CreateMessageRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, String)> {
    if req.content.is_empty() || req.content.len() > 2000 {
        return Err((StatusCode::BAD_REQUEST, "Invalid message content".to_string()));
    }

    let message_id = generate_id();
    let reply_to = req.reply_to_id.and_then(|id| id.parse::<i64>().ok().map(MessageId::new));

    let message = MessageRepository::create(
        &state.db.pg,
        message_id,
        ChannelId::new(channel_id),
        claims.user_id(),
        &req.content,
        reply_to,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = MessageResponse {
        id: message.id.to_string(),
        channel_id: message.channel_id.to_string(),
        author_id: message.author_id.to_string(),
        content: message.content.clone(),
        message_type: message.message_type,
        reply_to_id: message.reply_to_id.map(|id| id.to_string()),
        edited_at: message.edited_at.map(|t| t.to_rfc3339()),
        created_at: message.created_at.to_rfc3339(),
    };

    // Broadcast to gateway
    state.gateway.broadcast_message_create(channel_id, &response).await;

    Ok(Json(response))
}

/// Get single message
async fn get_message(
    State(state): State<MessagingState>,
    Extension(_claims): Extension<Claims>,
    Path((channel_id, message_id)): Path<(i64, i64)>,
) -> Result<Json<MessageResponse>, (StatusCode, String)> {
    let message = MessageRepository::find_by_id(&state.db.pg, MessageId::new(message_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Message not found".to_string()))?;

    if message.channel_id != channel_id {
        return Err((StatusCode::NOT_FOUND, "Message not found".to_string()));
    }

    Ok(Json(MessageResponse {
        id: message.id.to_string(),
        channel_id: message.channel_id.to_string(),
        author_id: message.author_id.to_string(),
        content: message.content,
        message_type: message.message_type,
        reply_to_id: message.reply_to_id.map(|id| id.to_string()),
        edited_at: message.edited_at.map(|t| t.to_rfc3339()),
        created_at: message.created_at.to_rfc3339(),
    }))
}

/// Update message
async fn update_message(
    State(state): State<MessagingState>,
    Extension(claims): Extension<Claims>,
    Path((_channel_id, message_id)): Path<(i64, i64)>,
    Json(req): Json<UpdateMessageRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, String)> {
    // Check author
    let existing = MessageRepository::find_by_id(&state.db.pg, MessageId::new(message_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Message not found".to_string()))?;

    if existing.author_id != claims.user_id().as_i64() {
        return Err((StatusCode::FORBIDDEN, "Not your message".to_string()));
    }

    let message = MessageRepository::update_content(&state.db.pg, MessageId::new(message_id), &req.content)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(MessageResponse {
        id: message.id.to_string(),
        channel_id: message.channel_id.to_string(),
        author_id: message.author_id.to_string(),
        content: message.content,
        message_type: message.message_type,
        reply_to_id: message.reply_to_id.map(|id| id.to_string()),
        edited_at: message.edited_at.map(|t| t.to_rfc3339()),
        created_at: message.created_at.to_rfc3339(),
    }))
}

/// Delete message
async fn delete_message(
    State(state): State<MessagingState>,
    Extension(claims): Extension<Claims>,
    Path((_channel_id, message_id)): Path<(i64, i64)>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Check author
    let existing = MessageRepository::find_by_id(&state.db.pg, MessageId::new(message_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Message not found".to_string()))?;

    if existing.author_id != claims.user_id().as_i64() {
        return Err((StatusCode::FORBIDDEN, "Not your message".to_string()));
    }

    MessageRepository::soft_delete(&state.db.pg, MessageId::new(message_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
