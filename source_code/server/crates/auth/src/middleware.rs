//! Authentication middleware

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::{Claims, JwtService};

#[derive(Clone)]
pub struct AuthMiddleware {
    jwt_service: Arc<JwtService>,
}

impl AuthMiddleware {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }
}

/// Extract JWT claims from request
pub async fn auth_middleware<B>(
    State(jwt_service): State<Arc<JwtService>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    match jwt_service.validate_access_token(token) {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

/// Optional auth - doesn't fail if no token
pub async fn optional_auth_middleware<B>(
    State(jwt_service): State<Arc<JwtService>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    if let Some(auth_header) = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
    {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            if let Ok(claims) = jwt_service.validate_access_token(token) {
                request.extensions_mut().insert(claims);
            }
        }
    }

    next.run(request).await
}

/// Extension trait to get claims from request
pub trait ClaimsExt {
    fn claims(&self) -> Option<&Claims>;
}

impl<B> ClaimsExt for Request<B> {
    fn claims(&self) -> Option<&Claims> {
        self.extensions().get::<Claims>()
    }
}
