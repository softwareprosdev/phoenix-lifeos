use chrono::Utc;
use phoenix_core::entities::{VaultAccessLog, VaultItem, VaultItemType};
use phoenix_db::repositories::*;
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;

// ─── Payloads ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateVaultItemPayload {
    pub item_type: VaultItemType,
    pub title: String,
    pub encrypted_data: String,
    pub key_version: Option<i32>,
    pub metadata: Option<String>,
}

// ─── Commands ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_vault_items(
    state: State<'_, AppState>,
    item_type: Option<String>,
) -> Result<Vec<VaultItem>, String> {
    let repo = SqlVaultItemRepository::new(state.pool.clone());
    match item_type {
        Some(t) => {
            let item_type = match t.as_str() {
                "password" => VaultItemType::Password,
                "note" => VaultItemType::Note,
                "key" => VaultItemType::Key,
                "secret" => VaultItemType::Secret,
                other => return Err(format!("invalid vault item_type: {other}")),
            };
            repo.list_by_type(item_type)
                .await
                .map_err(|e| e.to_string())
        }
        None => {
            let mut all = Vec::new();
            for t in &["password", "note", "key", "secret"] {
                let item_type = match *t {
                    "password" => VaultItemType::Password,
                    "note" => VaultItemType::Note,
                    "key" => VaultItemType::Key,
                    _ => VaultItemType::Secret,
                };
                if let Ok(more) = repo.list_by_type(item_type).await {
                    all.extend(more);
                }
            }
            Ok(all)
        }
    }
}

#[tauri::command]
pub async fn get_vault_item(state: State<'_, AppState>, id: String) -> Result<VaultItem, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlVaultItemRepository::new(state.pool.clone());
    repo.get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "vault item not found".to_string())
}

#[tauri::command]
pub async fn create_vault_item(
    state: State<'_, AppState>,
    payload: CreateVaultItemPayload,
) -> Result<VaultItem, String> {
    let repo = SqlVaultItemRepository::new(state.pool.clone());
    let now = Utc::now();
    let item = VaultItem {
        id: Uuid::new_v4(),
        item_type: payload.item_type,
        title: payload.title,
        encrypted_data: payload.encrypted_data,
        key_version: payload.key_version.unwrap_or(1),
        metadata: payload.metadata.unwrap_or_else(|| "{}".into()),
        created_at: now,
        updated_at: now,
    };
    repo.create(item).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_vault_item(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlVaultItemRepository::new(state.pool.clone());
    repo.delete(uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_vault_access_log(
    state: State<'_, AppState>,
    item_id: Option<String>,
) -> Result<Vec<VaultAccessLog>, String> {
    let log_repo = SqlVaultAccessLogRepository::new(state.pool.clone());
    match item_id {
        Some(iid) => {
            let uuid = Uuid::parse_str(&iid).map_err(|e| format!("invalid item_id: {e}"))?;
            log_repo
                .list_by_item(uuid)
                .await
                .map_err(|e| e.to_string())
        }
        None => log_repo.list_recent(100).await.map_err(|e| e.to_string()),
    }
}
