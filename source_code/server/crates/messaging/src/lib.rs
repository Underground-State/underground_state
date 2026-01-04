//! Messaging module with REST API and WebSocket gateway

pub mod gateway;
pub mod routes;

pub use gateway::Gateway;
pub use routes::messaging_router;
