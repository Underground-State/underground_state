//! User routes

use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use ug_auth::Claims;
use ug_db::{repositories::UserRepository, DbState};

#[derive(Clone)]
pub struct UsersState {
    pub db: DbState,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub wallet_address: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_hash: Option<String>,
    pub kyc_status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub display_name: Option<String>,
}

pub fn users_router() -> Router<UsersState> {
    Router::new()
        .route("/@me", get(get_current_user).patch(update_current_user))
        .route("/:user_id", get(get_user))
}

/// Get current authenticated user
async fn get_current_user(
    State(state): State<UsersState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = UserRepository::find_by_id(&state.db.pg, claims.user_id())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        wallet_address: user.wallet_address,
        username: user.username,
        display_name: user.display_name,
        avatar_hash: user.avatar_hash,
        kyc_status: user.kyc_status,
    }))
}

/// Update current user
async fn update_current_user(
    State(state): State<UsersState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = if let Some(username) = req.username {
        UserRepository::update_username(&state.db.pg, claims.user_id(), &username)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    } else {
        UserRepository::find_by_id(&state.db.pg, claims.user_id())
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?
    };

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        wallet_address: user.wallet_address,
        username: user.username,
        display_name: user.display_name,
        avatar_hash: user.avatar_hash,
        kyc_status: user.kyc_status,
    }))
}

/// Get user by ID
async fn get_user(
    State(state): State<UsersState>,
    Path(user_id): Path<i64>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = UserRepository::find_by_id(&state.db.pg, user_id.into())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        wallet_address: user.wallet_address,
        username: user.username,
        display_name: user.display_name,
        avatar_hash: user.avatar_hash,
        kyc_status: user.kyc_status,
    }))
}
