//! hCaptcha verification

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaptchaError {
    #[error("Captcha verification failed")]
    VerificationFailed,
    #[error("Invalid captcha response")]
    InvalidResponse,
    #[error("Network error: {0}")]
    NetworkError(String),
}

#[derive(Debug, Serialize)]
struct VerifyRequest {
    secret: String,
    response: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    remoteip: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VerifyResponse {
    success: bool,
    #[serde(default)]
    challenge_ts: Option<String>,
    #[serde(default)]
    hostname: Option<String>,
    #[serde(rename = "error-codes", default)]
    error_codes: Vec<String>,
}

pub struct HCaptchaService {
    secret: String,
    verify_url: String,
}

impl HCaptchaService {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            verify_url: "https://hcaptcha.com/siteverify".to_string(),
        }
    }

    /// Verify an hCaptcha token
    pub async fn verify(&self, token: &str, remote_ip: Option<&str>) -> Result<bool, CaptchaError> {
        let client = reqwest::Client::new();

        let mut params = vec![
            ("secret", self.secret.clone()),
            ("response", token.to_string()),
        ];

        if let Some(ip) = remote_ip {
            params.push(("remoteip", ip.to_string()));
        }

        let response = client
            .post(&self.verify_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| CaptchaError::NetworkError(e.to_string()))?;

        let verify_response: VerifyResponse = response
            .json()
            .await
            .map_err(|_| CaptchaError::InvalidResponse)?;

        if verify_response.success {
            Ok(true)
        } else {
            tracing::warn!("hCaptcha verification failed: {:?}", verify_response.error_codes);
            Err(CaptchaError::VerificationFailed)
        }
    }
}

/// Mock captcha service for development
pub struct MockCaptchaService;

impl MockCaptchaService {
    pub async fn verify(&self, token: &str, _remote_ip: Option<&str>) -> Result<bool, CaptchaError> {
        // Accept any token in development
        if token == "test" || token.starts_with("10000000-") {
            Ok(true)
        } else {
            Ok(true) // Always pass in mock mode
        }
    }
}
