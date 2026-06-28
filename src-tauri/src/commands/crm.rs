use chrono::Utc;
use phoenix_core::entities::{Contact, ContactInteraction};
use phoenix_db::repositories::*;
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;

// ─── Payloads ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateContactPayload {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub organization: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateContactPayload {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub organization: Option<String>,
    pub notes: Option<String>,
}

// ─── Commands ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_contacts(state: State<'_, AppState>) -> Result<Vec<Contact>, String> {
    let repo = SqlContactRepository::new(state.pool.clone());
    repo.search("").await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_contact(
    state: State<'_, AppState>,
    payload: CreateContactPayload,
) -> Result<Contact, String> {
    let repo = SqlContactRepository::new(state.pool.clone());
    let now = Utc::now();
    let contact = Contact {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
        phone: payload.phone,
        organization: payload.organization,
        notes: payload.notes,
        created_at: now,
        updated_at: now,
        deleted_at: None,
    };
    repo.create(contact).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_contact(
    state: State<'_, AppState>,
    payload: UpdateContactPayload,
) -> Result<Contact, String> {
    let uuid = Uuid::parse_str(&payload.id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlContactRepository::new(state.pool.clone());
    let existing = repo
        .get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "contact not found".to_string())?;
    let now = Utc::now();
    let contact = Contact {
        id: uuid,
        name: payload.name,
        email: payload.email,
        phone: payload.phone,
        organization: payload.organization,
        notes: payload.notes,
        created_at: existing.created_at,
        updated_at: now,
        deleted_at: existing.deleted_at,
    };
    repo.update(contact).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_contact(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlContactRepository::new(state.pool.clone());
    repo.soft_delete(uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_contacts(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<Contact>, String> {
    let repo = SqlContactRepository::new(state.pool.clone());
    repo.search(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_contact_interactions(
    state: State<'_, AppState>,
    contact_id: String,
) -> Result<Vec<ContactInteraction>, String> {
    let uuid = Uuid::parse_str(&contact_id).map_err(|e| format!("invalid contact_id: {e}"))?;
    let repo = SqlContactInteractionRepository::new(state.pool.clone());
    repo.list_by_contact(uuid)
        .await
        .map_err(|e| e.to_string())
}
