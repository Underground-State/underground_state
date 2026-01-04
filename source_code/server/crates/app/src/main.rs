//! Underground State - Main Application Entry Point

use anyhow::Result;
use axum::{
    middleware,
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod state;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    tracing::info!("Starting Underground State server...");

    // Initialize application state
    let state = AppState::new(&config).await?;

    // Build router
    let app = create_router(state.clone());

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // API v1 routes
    let api_v1 = Router::new()
        // Auth routes (no auth required)
        .nest("/auth", ug_auth::auth_router().with_state(state.auth_state.clone()))
        // Protected routes
        .nest(
            "/users",
            ug_users::users_router()
                .with_state(state.users_state.clone())
                .layer(middleware::from_fn_with_state(
                    state.jwt_service.clone(),
                    ug_auth::middleware::auth_middleware,
                )),
        )
        .nest(
            "/guilds",
            ug_guilds::guilds_router()
                .with_state(state.guilds_state.clone())
                .layer(middleware::from_fn_with_state(
                    state.jwt_service.clone(),
                    ug_auth::middleware::auth_middleware,
                )),
        )
        .nest(
            "/channels",
            ug_channels::channels_router()
                .with_state(state.channels_state.clone())
                .layer(middleware::from_fn_with_state(
                    state.jwt_service.clone(),
                    ug_auth::middleware::auth_middleware,
                )),
        )
        .merge(
            ug_messaging::messaging_router()
                .with_state(state.messaging_state.clone())
                .layer(middleware::from_fn_with_state(
                    state.jwt_service.clone(),
                    ug_auth::middleware::auth_middleware,
                )),
        )
        .nest(
            "/kyc",
            ug_kyc::kyc_router()
                .with_state(state.kyc_state.clone())
                .layer(middleware::from_fn_with_state(
                    state.jwt_service.clone(),
                    ug_auth::middleware::auth_middleware,
                )),
        );

    // WebSocket routes
    let ws_routes = Router::new()
        .route(
            "/gateway",
            get(ug_messaging::Gateway::handle_upgrade).with_state(state.gateway.clone()),
        )
        .route(
            "/voice/:guild_id/:channel_id",
            get(ug_voice::VoiceSignaling::handle_upgrade).with_state(state.voice_signaling.clone()),
        );

    // Combine all routes
    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", api_v1)
        .nest("/ws", ws_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}

async fn health_check() -> &'static str {
    "OK"
}
