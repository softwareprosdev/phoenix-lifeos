use phoenix_core::entities::{Document, DocumentSource};
use phoenix_vault::{CreateDocumentInput, DocumentService, UpdateDocumentInput};
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateDocumentPayload {
    pub name: String,
    pub file_type: String,
    pub mime_type: Option<String>,
    pub encrypted_path: String,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateDocumentPayload {
    pub name: String,
    pub file_type: String,
    pub mime_type: Option<String>,
    pub encrypted_path: String,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<String>,
}

fn service(state: &AppState) -> DocumentService {
    DocumentService::new(state.pool.clone())
}

fn parse_source(s: Option<String>) -> Result<DocumentSource, String> {
    match s.as_deref() {
        None | Some("upload") => Ok(DocumentSource::Upload),
        Some("email") => Ok(DocumentSource::Email),
        Some("scan") => Ok(DocumentSource::Scan),
        Some(other) => Err(format!("invalid document source: {other}")),
    }
}

#[tauri::command]
pub async fn list_documents(state: State<'_, AppState>) -> Result<Vec<Document>, String> {
    let svc = service(&state);
    svc.list_all().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_document(state: State<'_, AppState>, id: String) -> Result<Document, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let svc = service(&state);
    svc.get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "document not found".to_string())
}

#[tauri::command]
pub async fn create_document(
    state: State<'_, AppState>,
    payload: CreateDocumentPayload,
) -> Result<Document, String> {
    let svc = service(&state);
    let input = CreateDocumentInput {
        name: payload.name,
        file_type: payload.file_type,
        mime_type: payload.mime_type,
        encrypted_path: payload.encrypted_path,
        source: parse_source(payload.source)?,
        tags: payload.tags.unwrap_or_default(),
        metadata: payload.metadata.unwrap_or_else(|| "{}".into()),
    };
    svc.create(input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_document(
    state: State<'_, AppState>,
    id: String,
    payload: UpdateDocumentPayload,
) -> Result<Document, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let svc = service(&state);
    let input = UpdateDocumentInput {
        id: uuid,
        name: payload.name,
        file_type: payload.file_type,
        mime_type: payload.mime_type,
        encrypted_path: payload.encrypted_path,
        source: parse_source(payload.source)?,
        tags: payload.tags.unwrap_or_default(),
        metadata: payload.metadata.unwrap_or_else(|| "{}".into()),
    };
    svc.update(input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_document(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let svc = service(&state);
    svc.delete(uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_documents(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<Document>, String> {
    let svc = service(&state);
    svc.search(&query).await.map_err(|e| e.to_string())
}
