//! Error types for the application

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // Authentication errors
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid nonce")]
    InvalidNonce,

    #[error("Token expired")]
    TokenExpired,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Captcha verification failed")]
    CaptchaFailed,

    // Resource errors
    #[error("User not found")]
    UserNotFound,

    #[error("Guild not found")]
    GuildNotFound,

    #[error("Channel not found")]
    ChannelNotFound,

    #[error("Message not found")]
    MessageNotFound,

    #[error("Resource already exists")]
    AlreadyExists,

    // Permission errors
    #[error("Permission denied")]
    PermissionDenied,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    // KYC errors
    #[error("KYC required")]
    KycRequired,

    #[error("KYC pending")]
    KycPending,

    #[error("KYC rejected")]
    KycRejected,

    // Database errors
    #[error("Database error: {0}")]
    Database(String),

    // External service errors
    #[error("External service error: {0}")]
    ExternalService(String),

    // Internal errors
    #[error("Internal server error")]
    Internal,

    #[error("{0}")]
    Custom(String),
}

impl Error {
    pub fn status_code(&self) -> u16 {
        match self {
            Error::InvalidCredentials
            | Error::InvalidSignature
            | Error::InvalidNonce
            | Error::CaptchaFailed => 401,

            Error::TokenExpired | Error::Unauthorized => 401,

            Error::PermissionDenied | Error::InsufficientPermissions => 403,

            Error::KycRequired | Error::KycPending | Error::KycRejected => 403,

            Error::UserNotFound
            | Error::GuildNotFound
            | Error::ChannelNotFound
            | Error::MessageNotFound => 404,

            Error::AlreadyExists => 409,

            Error::Validation(_) | Error::InvalidInput(_) => 400,

            Error::Database(_) | Error::ExternalService(_) | Error::Internal => 500,

            Error::Custom(_) => 500,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Error::InvalidCredentials => "INVALID_CREDENTIALS",
            Error::InvalidSignature => "INVALID_SIGNATURE",
            Error::InvalidNonce => "INVALID_NONCE",
            Error::TokenExpired => "TOKEN_EXPIRED",
            Error::Unauthorized => "UNAUTHORIZED",
            Error::CaptchaFailed => "CAPTCHA_FAILED",
            Error::UserNotFound => "USER_NOT_FOUND",
            Error::GuildNotFound => "GUILD_NOT_FOUND",
            Error::ChannelNotFound => "CHANNEL_NOT_FOUND",
            Error::MessageNotFound => "MESSAGE_NOT_FOUND",
            Error::AlreadyExists => "ALREADY_EXISTS",
            Error::PermissionDenied => "PERMISSION_DENIED",
            Error::InsufficientPermissions => "INSUFFICIENT_PERMISSIONS",
            Error::Validation(_) => "VALIDATION_ERROR",
            Error::InvalidInput(_) => "INVALID_INPUT",
            Error::KycRequired => "KYC_REQUIRED",
            Error::KycPending => "KYC_PENDING",
            Error::KycRejected => "KYC_REJECTED",
            Error::Database(_) => "DATABASE_ERROR",
            Error::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
            Error::Internal => "INTERNAL_ERROR",
            Error::Custom(_) => "CUSTOM_ERROR",
        }
    }
}
