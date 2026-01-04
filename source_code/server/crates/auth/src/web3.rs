//! Web3 SIWE (Sign-In with Ethereum) authentication

use ethers::types::{Address, Signature};
use siwe::{Message, VerificationOpts};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Web3Error {
    #[error("Invalid wallet address")]
    InvalidAddress,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Signature verification failed")]
    VerificationFailed,
    #[error("Message parsing failed")]
    MessageParseFailed,
}

pub struct SiweService {
    domain: String,
    uri: String,
}

impl SiweService {
    pub fn new(domain: String, uri: String) -> Self {
        Self { domain, uri }
    }

    /// Create a SIWE message for the user to sign
    pub fn create_message(&self, wallet_address: &str, nonce: &str) -> Result<String, Web3Error> {
        let address = Address::from_str(wallet_address).map_err(|_| Web3Error::InvalidAddress)?;

        let message = format!(
            "{domain} wants you to sign in with your Ethereum account:
{address}

Sign in to Underground State

URI: {uri}
Version: 1
Chain ID: 1
Nonce: {nonce}
Issued At: {issued_at}",
            domain = self.domain,
            address = format!("{:?}", address),
            uri = self.uri,
            nonce = nonce,
            issued_at = chrono::Utc::now().to_rfc3339()
        );

        Ok(message)
    }

    /// Verify a signed message
    pub async fn verify_signature(
        &self,
        message: &str,
        signature: &str,
        expected_nonce: &str,
    ) -> Result<String, Web3Error> {
        // Parse the SIWE message
        let siwe_message: Message = message.parse().map_err(|_| Web3Error::MessageParseFailed)?;

        // Parse signature
        let sig = Signature::from_str(signature.trim_start_matches("0x"))
            .map_err(|_| Web3Error::InvalidSignature)?;

        // Verify the signature
        let opts = VerificationOpts {
            domain: Some(self.domain.clone().try_into().unwrap()),
            nonce: Some(expected_nonce.to_string()),
            timestamp: Some(chrono::Utc::now().into()),
        };

        siwe_message
            .verify(&sig.to_vec(), &opts)
            .await
            .map_err(|_| Web3Error::VerificationFailed)?;

        // Return the recovered address
        Ok(format!("{:?}", siwe_message.address))
    }

    /// Verify signature for any chain (simplified)
    pub fn verify_eth_signature(
        message: &str,
        signature: &str,
        expected_address: &str,
    ) -> Result<bool, Web3Error> {
        let sig = Signature::from_str(signature.trim_start_matches("0x"))
            .map_err(|_| Web3Error::InvalidSignature)?;

        let message_hash = ethers::utils::hash_message(message);
        let recovered = sig
            .recover(message_hash)
            .map_err(|_| Web3Error::VerificationFailed)?;

        let expected =
            Address::from_str(expected_address).map_err(|_| Web3Error::InvalidAddress)?;

        Ok(recovered == expected)
    }
}

/// Generate a random nonce
pub fn generate_nonce() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.gen();
    hex::encode(bytes)
}
