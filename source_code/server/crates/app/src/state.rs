//! Application state

use anyhow::Result;
use std::sync::Arc;

use ug_auth::{captcha::HCaptchaService, jwt::JwtService, routes::AuthState, web3::SiweService};
use ug_channels::routes::ChannelsState;
use ug_db::DbState;
use ug_guilds::routes::GuildsState;
use ug_kyc::{encryption::KycEncryption, routes::KycState, storage::LocalKycStorage};
use ug_messaging::{routes::MessagingState, Gateway};
use ug_users::routes::UsersState;
use ug_voice::VoiceSignaling;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: DbState,
    pub jwt_service: Arc<JwtService>,
    pub gateway: Arc<Gateway>,
    pub voice_signaling: Arc<VoiceSignaling>,

    // Feature states
    pub auth_state: AuthState,
    pub users_state: UsersState,
    pub guilds_state: GuildsState,
    pub channels_state: ChannelsState,
    pub messaging_state: MessagingState,
    pub kyc_state: KycState,
}

impl AppState {
    pub async fn new(config: &Config) -> Result<Self> {
        // Initialize database connections
        let db = DbState::new(&config.database_url, &config.redis_url).await?;

        // Initialize services
        let jwt_service = Arc::new(JwtService::new(&config.jwt_secret));
        let siwe_service = Arc::new(SiweService::new(
            config.siwe_domain.clone(),
            config.siwe_uri.clone(),
        ));
        let captcha_service = Arc::new(HCaptchaService::new(config.hcaptcha_secret.clone()));

        // Initialize gateway
        let gateway = Arc::new(Gateway::new(jwt_service.clone()));

        // Initialize voice signaling
        let voice_signaling = Arc::new(VoiceSignaling::new());

        // Initialize KYC storage
        let kyc_encryption = KycEncryption::from_hex(&config.kyc_encryption_key)?;
        let kyc_storage = Arc::new(LocalKycStorage::new(&config.kyc_storage_path, kyc_encryption));

        // Build feature states
        let auth_state = AuthState {
            db: db.clone(),
            jwt: jwt_service.clone(),
            siwe: siwe_service,
            captcha: captcha_service,
        };

        let users_state = UsersState { db: db.clone() };
        let guilds_state = GuildsState { db: db.clone() };
        let channels_state = ChannelsState { db: db.clone() };
        let messaging_state = MessagingState {
            db: db.clone(),
            gateway: gateway.clone(),
        };
        let kyc_state = KycState {
            db: db.clone(),
            storage: kyc_storage,
        };

        Ok(Self {
            db,
            jwt_service,
            gateway,
            voice_signaling,
            auth_state,
            users_state,
            guilds_state,
            channels_state,
            messaging_state,
            kyc_state,
        })
    }
}
