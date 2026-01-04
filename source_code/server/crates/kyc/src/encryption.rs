//! Document encryption using AES-256-GCM

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::Rng;
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Invalid key")]
    InvalidKey,
}

pub struct KycEncryption {
    master_key: [u8; 32],
}

impl KycEncryption {
    pub fn new(master_key: &[u8]) -> Result<Self, EncryptionError> {
        if master_key.len() != 32 {
            return Err(EncryptionError::InvalidKey);
        }
        let mut key = [0u8; 32];
        key.copy_from_slice(master_key);
        Ok(Self { master_key: key })
    }

    pub fn from_hex(hex_key: &str) -> Result<Self, EncryptionError> {
        let bytes = hex::decode(hex_key).map_err(|_| EncryptionError::InvalidKey)?;
        Self::new(&bytes)
    }

    /// Encrypt document data
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedDocument, EncryptionError> {
        let cipher = Aes256Gcm::new_from_slice(&self.master_key)
            .map_err(|_| EncryptionError::InvalidKey)?;

        // Generate random 96-bit nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| EncryptionError::EncryptionFailed)?;

        // Hash original for integrity
        let mut hasher = Sha256::new();
        hasher.update(plaintext);
        let hash = hex::encode(hasher.finalize());

        Ok(EncryptedDocument {
            ciphertext,
            nonce: nonce_bytes,
            original_hash: hash,
        })
    }

    /// Decrypt document data
    pub fn decrypt(&self, doc: &EncryptedDocument) -> Result<Vec<u8>, EncryptionError> {
        let cipher = Aes256Gcm::new_from_slice(&self.master_key)
            .map_err(|_| EncryptionError::InvalidKey)?;

        let nonce = Nonce::from_slice(&doc.nonce);

        let plaintext = cipher
            .decrypt(nonce, doc.ciphertext.as_slice())
            .map_err(|_| EncryptionError::DecryptionFailed)?;

        // Verify hash
        let mut hasher = Sha256::new();
        hasher.update(&plaintext);
        let hash = hex::encode(hasher.finalize());

        if hash != doc.original_hash {
            return Err(EncryptionError::DecryptionFailed);
        }

        Ok(plaintext)
    }
}

#[derive(Debug, Clone)]
pub struct EncryptedDocument {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub original_hash: String,
}

impl EncryptedDocument {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.nonce);
        result.extend_from_slice(&(self.ciphertext.len() as u32).to_le_bytes());
        result.extend_from_slice(&self.ciphertext);
        result.extend_from_slice(self.original_hash.as_bytes());
        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, EncryptionError> {
        if bytes.len() < 12 + 4 + 64 {
            return Err(EncryptionError::DecryptionFailed);
        }

        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&bytes[0..12]);

        let ciphertext_len = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as usize;

        if bytes.len() < 16 + ciphertext_len + 64 {
            return Err(EncryptionError::DecryptionFailed);
        }

        let ciphertext = bytes[16..16 + ciphertext_len].to_vec();
        let original_hash = String::from_utf8_lossy(&bytes[16 + ciphertext_len..]).to_string();

        Ok(Self {
            ciphertext,
            nonce,
            original_hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32];
        let encryption = KycEncryption::new(&key).unwrap();

        let plaintext = b"Hello, World!";
        let encrypted = encryption.encrypt(plaintext).unwrap();
        let decrypted = encryption.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }
}
