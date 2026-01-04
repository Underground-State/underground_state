//! Authentication routes

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{captcha::HCaptchaService, jwt::JwtService, web3::{generate_nonce, SiweService}};
use ug_common::snowflake::generate_id;
use ug_db::{repositories::UserRepository, DbState};

/// Auth state containing all services
#[derive(Clone)]
pub struct AuthState {
    pub db: DbState,
    pub jwt: Arc<JwtService>,
    pub siwe: Arc<SiweService>,
    pub captcha: Arc<HCaptchaService>,
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct NonceRequest {
    pub wallet_address: String,
}

#[derive(Debug, Serialize)]
pub struct NonceResponse {
    pub nonce: String,
    pub message: String,
    pub expires_at: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub wallet_address: String,
    pub signature: String,
    pub message: String,
    pub nonce: String,
    pub hcaptcha_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub wallet_address: String,
    pub username: String,
    pub kyc_status: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Create auth router
pub fn auth_router() -> Router<AuthState> {
    Router::new()
        .route("/nonce", post(get_nonce))
        .route("/verify", post(verify_signature))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
}

/// Get nonce for SIWE
async fn get_nonce(
    State(state): State<AuthState>,
    Json(req): Json<NonceRequest>,
) -> Result<Json<NonceResponse>, (StatusCode, String)> {
    let nonce = generate_nonce();
    let message = state
        .siwe
        .create_message(&req.wallet_address, &nonce)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Store nonce in Redis (5 minute expiry)
    let nonce_key = format!("nonce:{}", nonce);
    state
        .db
        .redis
        .set_ex(&nonce_key, &req.wallet_address.to_lowercase(), 300)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let expires_at = (chrono::Utc::now() + chrono::Duration::minutes(5)).to_rfc3339();

    Ok(Json(NonceResponse {
        nonce,
        message,
        expires_at,
    }))
}

/// Verify signature and authenticate
async fn verify_signature(
    State(state): State<AuthState>,
    Json(req): Json<VerifyRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // Verify hCaptcha
    state
        .captcha
        .verify(&req.hcaptcha_token, None)
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, "Captcha verification failed".to_string()))?;

    // Check nonce exists and matches
    let nonce_key = format!("nonce:{}", req.nonce);
    let stored_wallet = state
        .db
        .redis
        .get_key(&nonce_key)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::BAD_REQUEST, "Invalid or expired nonce".to_string()))?;

    if stored_wallet.to_lowercase() != req.wallet_address.to_lowercase() {
        return Err((StatusCode::BAD_REQUEST, "Nonce mismatch".to_string()));
    }

    // Verify signature
    state
        .siwe
        .verify_signature(&req.message, &req.signature, &req.nonce)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // Delete used nonce
    let _ = state.db.redis.del(&nonce_key).await;

    // Find or create user
    let wallet_lower = req.wallet_address.to_lowercase();
    let user = match UserRepository::find_by_wallet(&state.db.pg, &wallet_lower).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Create new user
            let user_id = generate_id();
            let username = format!("user_{}", &wallet_lower[2..10]);
            UserRepository::create(&state.db.pg, user_id, &wallet_lower, &username)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        }
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    // Generate tokens
    let user_id = ug_core::UserId::new(user.id);
    let tokens = state
        .jwt
        .generate_token_pair(user_id, &user.wallet_address)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: tokens.expires_in,
        token_type: tokens.token_type,
        user: UserResponse {
            id: user.id.to_string(),
            wallet_address: user.wallet_address,
            username: user.username,
            kyc_status: user.kyc_status,
        },
    }))
}

/// Refresh access token
async fn refresh_token(
    State(state): State<AuthState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let claims = state
        .jwt
        .validate_refresh_token(&req.refresh_token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid refresh token".to_string()))?;

    let user = UserRepository::find_by_id(&state.db.pg, claims.user_id())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let tokens = state
        .jwt
        .generate_token_pair(claims.user_id(), &user.wallet_address)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: tokens.expires_in,
        token_type: tokens.token_type,
        user: UserResponse {
            id: user.id.to_string(),
            wallet_address: user.wallet_address,
            username: user.username,
            kyc_status: user.kyc_status,
        },
    }))
}

/// Logout (invalidate tokens)
async fn logout(
    State(_state): State<AuthState>,
) -> impl IntoResponse {
    // In a production app, you'd add the token to a blocklist in Redis
    StatusCode::NO_CONTENT
}
