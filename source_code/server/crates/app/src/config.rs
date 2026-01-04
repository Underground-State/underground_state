//! Application configuration

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    // Server
    pub host: String,
    pub port: u16,

    // Database
    pub database_url: String,
    pub redis_url: String,

    // Auth
    pub jwt_secret: String,
    pub hcaptcha_secret: String,

    // SIWE
    pub siwe_domain: String,
    pub siwe_uri: String,

    // KYC
    pub kyc_storage_path: String,
    pub kyc_encryption_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("Invalid PORT")?,

            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL is required")?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),

            jwt_secret: std::env::var("JWT_SECRET")
                .context("JWT_SECRET is required")?,
            hcaptcha_secret: std::env::var("HCAPTCHA_SECRET")
                .unwrap_or_else(|_| "test".to_string()),

            siwe_domain: std::env::var("SIWE_DOMAIN")
                .unwrap_or_else(|_| "localhost".to_string()),
            siwe_uri: std::env::var("SIWE_URI")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),

            kyc_storage_path: std::env::var("KYC_STORAGE_PATH")
                .unwrap_or_else(|_| "./data/kyc".to_string()),
            kyc_encryption_key: std::env::var("KYC_ENCRYPTION_KEY")
                .unwrap_or_else(|_| {
                    // Generate a default key for development (32 bytes = 64 hex chars)
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string()
                }),
        })
    }
}
