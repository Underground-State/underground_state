//! JWT token generation and validation

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ug_core::UserId;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Token creation failed")]
    CreationFailed,
    #[error("Token validation failed")]
    ValidationFailed,
    #[error("Token expired")]
    Expired,
    #[error("Invalid token")]
    Invalid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Wallet address
    pub wallet: String,
    /// Issued at
    pub iat: i64,
    /// Expiration
    pub exp: i64,
    /// Token type (access or refresh)
    pub token_type: String,
}

impl Claims {
    pub fn user_id(&self) -> UserId {
        UserId::new(self.sub.parse().unwrap_or(0))
    }
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_duration: Duration,
    refresh_token_duration: Duration,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_token_duration: Duration::hours(1),
            refresh_token_duration: Duration::days(7),
        }
    }

    /// Generate an access token
    pub fn generate_access_token(
        &self,
        user_id: UserId,
        wallet_address: &str,
    ) -> Result<String, JwtError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.as_i64().to_string(),
            wallet: wallet_address.to_lowercase(),
            iat: now.timestamp(),
            exp: (now + self.access_token_duration).timestamp(),
            token_type: "access".to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| JwtError::CreationFailed)
    }

    /// Generate a refresh token
    pub fn generate_refresh_token(
        &self,
        user_id: UserId,
        wallet_address: &str,
    ) -> Result<String, JwtError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.as_i64().to_string(),
            wallet: wallet_address.to_lowercase(),
            iat: now.timestamp(),
            exp: (now + self.refresh_token_duration).timestamp(),
            token_type: "refresh".to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| JwtError::CreationFailed)
    }

    /// Validate a token and extract claims
    pub fn validate(&self, token: &str) -> Result<Claims, JwtError> {
        let validation = Validation::default();

        let token_data: TokenData<Claims> = decode(token, &self.decoding_key, &validation)
            .map_err(|e| {
                tracing::debug!("JWT validation error: {:?}", e);
                match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::Expired,
                    _ => JwtError::Invalid,
                }
            })?;

        Ok(token_data.claims)
    }

    /// Validate access token specifically
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, JwtError> {
        let claims = self.validate(token)?;
        if claims.token_type != "access" {
            return Err(JwtError::Invalid);
        }
        Ok(claims)
    }

    /// Validate refresh token specifically
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, JwtError> {
        let claims = self.validate(token)?;
        if claims.token_type != "refresh" {
            return Err(JwtError::Invalid);
        }
        Ok(claims)
    }
}

#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

impl JwtService {
    pub fn generate_token_pair(
        &self,
        user_id: UserId,
        wallet_address: &str,
    ) -> Result<TokenPair, JwtError> {
        Ok(TokenPair {
            access_token: self.generate_access_token(user_id, wallet_address)?,
            refresh_token: self.generate_refresh_token(user_id, wallet_address)?,
            expires_in: self.access_token_duration.num_seconds(),
            token_type: "Bearer".to_string(),
        })
    }
}
