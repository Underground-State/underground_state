//! KYC routes

use axum::{
    extract::{Multipart, Path, State, Extension},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use ug_auth::Claims;
use ug_common::snowflake::generate_id;
use ug_db::DbState;

use crate::storage::LocalKycStorage;

#[derive(Clone)]
pub struct KycState {
    pub db: DbState,
    pub storage: Arc<LocalKycStorage>,
}

#[derive(Debug, Serialize)]
pub struct KycStatusResponse {
    pub status: String,
    pub documents: Vec<DocumentInfo>,
}

#[derive(Debug, Serialize)]
pub struct DocumentInfo {
    pub id: String,
    pub document_type: String,
    pub status: String,
    pub submitted_at: String,
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub id: String,
    pub document_type: String,
    pub status: String,
}

pub fn kyc_router() -> Router<KycState> {
    Router::new()
        .route("/status", get(get_kyc_status))
        .route("/documents", post(upload_document).get(list_documents))
}

/// Get KYC status
async fn get_kyc_status(
    State(state): State<KycState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<KycStatusResponse>, (StatusCode, String)> {
    // Get user's KYC status from database
    let user_id = claims.user_id().as_i64();

    let documents: Vec<(i64, String, String, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT id, document_type, status, submitted_at FROM kyc_documents WHERE user_id = $1 ORDER BY submitted_at DESC"
    )
    .bind(user_id)
    .fetch_all(&state.db.pg)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let status = if documents.is_empty() {
        "none"
    } else if documents.iter().any(|d| d.2 == "approved") {
        "approved"
    } else if documents.iter().any(|d| d.2 == "pending") {
        "pending"
    } else {
        "rejected"
    };

    Ok(Json(KycStatusResponse {
        status: status.to_string(),
        documents: documents
            .into_iter()
            .map(|(id, doc_type, status, submitted_at)| DocumentInfo {
                id: id.to_string(),
                document_type: doc_type,
                status,
                submitted_at: submitted_at.to_rfc3339(),
            })
            .collect(),
    }))
}

/// Upload KYC document
async fn upload_document(
    State(state): State<KycState>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, (StatusCode, String)> {
    let user_id = claims.user_id().as_i64();
    let mut document_type = String::new();
    let mut file_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "document_type" => {
                document_type = field
                    .text()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            }
            "file" => {
                file_data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
                        .to_vec(),
                );
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or((StatusCode::BAD_REQUEST, "No file provided".to_string()))?;

    if document_type.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Document type required".to_string()));
    }

    // Validate document type
    let valid_types = ["passport", "id_card", "drivers_license"];
    if !valid_types.contains(&document_type.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid document type".to_string()));
    }

    // Store encrypted file
    let metadata = state
        .storage
        .store(user_id, &document_type, &file_data)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Save to database
    let doc_id = generate_id();
    sqlx::query(
        "INSERT INTO kyc_documents (id, user_id, document_type, storage_path, file_hash, status, submitted_at)
         VALUES ($1, $2, $3, $4, $5, 'pending', NOW())"
    )
    .bind(doc_id)
    .bind(user_id)
    .bind(&document_type)
    .bind(&metadata.path)
    .bind(&metadata.file_hash)
    .execute(&state.db.pg)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Update user's KYC status
    sqlx::query("UPDATE users SET kyc_status = 'pending' WHERE id = $1 AND kyc_status = 'none'")
        .bind(user_id)
        .execute(&state.db.pg)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UploadResponse {
        id: doc_id.to_string(),
        document_type,
        status: "pending".to_string(),
    }))
}

/// List user's documents
async fn list_documents(
    State(state): State<KycState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<DocumentInfo>>, (StatusCode, String)> {
    let user_id = claims.user_id().as_i64();

    let documents: Vec<(i64, String, String, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT id, document_type, status, submitted_at FROM kyc_documents WHERE user_id = $1 ORDER BY submitted_at DESC"
    )
    .bind(user_id)
    .fetch_all(&state.db.pg)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(
        documents
            .into_iter()
            .map(|(id, doc_type, status, submitted_at)| DocumentInfo {
                id: id.to_string(),
                document_type: doc_type,
                status,
                submitted_at: submitted_at.to_rfc3339(),
            })
            .collect(),
    ))
}
