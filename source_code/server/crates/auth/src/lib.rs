//! Authentication module with Web3, hCaptcha, and JWT

pub mod captcha;
pub mod jwt;
pub mod middleware;
pub mod routes;
pub mod web3;

pub use jwt::{Claims, JwtService};
pub use middleware::AuthMiddleware;
pub use routes::auth_router;
