//! Local encrypted file storage for KYC documents

use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::encryption::{EncryptedDocument, EncryptionError, KycEncryption};

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Encryption error: {0}")]
    Encryption(#[from] EncryptionError),
    #[error("File not found")]
    NotFound,
}

pub struct LocalKycStorage {
    base_path: PathBuf,
    encryption: KycEncryption,
}

impl LocalKycStorage {
    pub fn new(base_path: impl AsRef<Path>, encryption: KycEncryption) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            encryption,
        }
    }

    /// Store encrypted document
    pub async fn store(
        &self,
        user_id: i64,
        doc_type: &str,
        data: &[u8],
    ) -> Result<StorageMetadata, StorageError> {
        // Encrypt the data
        let encrypted = self.encryption.encrypt(data)?;

        // Generate filename
        let filename = format!("{}.enc", uuid::Uuid::new_v4());
        let rel_path = format!("{}/{}/{}", user_id, doc_type, filename);
        let full_path = self.base_path.join(&rel_path);

        // Ensure directory exists
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Write encrypted data
        let mut file = fs::File::create(&full_path).await?;
        file.write_all(&encrypted.to_bytes()).await?;

        Ok(StorageMetadata {
            path: rel_path,
            file_hash: encrypted.original_hash,
            size: data.len(),
        })
    }

    /// Retrieve and decrypt document
    pub async fn retrieve(&self, path: &str) -> Result<Vec<u8>, StorageError> {
        let full_path = self.base_path.join(path);

        if !full_path.exists() {
            return Err(StorageError::NotFound);
        }

        let bytes = fs::read(&full_path).await?;
        let encrypted = EncryptedDocument::from_bytes(&bytes)?;
        let decrypted = self.encryption.decrypt(&encrypted)?;

        Ok(decrypted)
    }

    /// Delete document
    pub async fn delete(&self, path: &str) -> Result<(), StorageError> {
        let full_path = self.base_path.join(path);
        fs::remove_file(&full_path).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StorageMetadata {
    pub path: String,
    pub file_hash: String,
    pub size: usize,
}
