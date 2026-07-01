use phoenix_core::entities::{VaultAccessLog, VaultItem, VaultItemType};
use phoenix_vault::{CreateVaultItemInput, UpdateVaultItemInput, VaultService};
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateVaultItemPayload {
    pub item_type: VaultItemType,
    pub title: String,
    pub data: String,
    pub key_version: Option<i32>,
    pub metadata: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateVaultItemPayload {
    pub item_type: VaultItemType,
    pub title: String,
    pub data: String,
    pub key_version: Option<i32>,
    pub metadata: Option<String>,
}

fn service(state: &AppState) -> VaultService {
    VaultService::new(state.pool.clone(), state.vault_key)
}

#[tauri::command]
pub async fn list_vault_items(
    state: State<'_, AppState>,
    item_type: Option<String>,
) -> Result<Vec<VaultItem>, String> {
    let svc = service(&state);
    match item_type {
        Some(t) => {
            let vtype = parse_vault_item_type(&t)?;
            svc.list_by_type(vtype).await.map_err(|e| e.to_string())
        }
        None => svc.list_all().await.map_err(|e| e.to_string()),
    }
}

#[tauri::command]
pub async fn get_vault_item(state: State<'_, AppState>, id: String) -> Result<VaultItem, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let svc = service(&state);
    svc.get_item(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "vault item not found".to_string())
}

#[tauri::command]
pub async fn create_vault_item(
    state: State<'_, AppState>,
    payload: CreateVaultItemPayload,
) -> Result<VaultItem, String> {
    let svc = service(&state);
    let input = CreateVaultItemInput {
        item_type: payload.item_type,
        title: payload.title,
        data: payload.data,
        key_version: payload.key_version.unwrap_or(1),
        metadata: payload.metadata.unwrap_or_else(|| "{}".into()),
    };
    svc.create_item(input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_vault_item(
    state: State<'_, AppState>,
    id: String,
    payload: UpdateVaultItemPayload,
) -> Result<VaultItem, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let svc = service(&state);
    let input = UpdateVaultItemInput {
        id: uuid,
        item_type: payload.item_type,
        title: payload.title,
        data: payload.data,
        key_version: payload.key_version.unwrap_or(1),
        metadata: payload.metadata.unwrap_or_else(|| "{}".into()),
    };
    svc.update_item(input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_vault_item(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let svc = service(&state);
    svc.delete_item(uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_vault_items(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<VaultItem>, String> {
    let svc = service(&state);
    svc.search(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_vault_access_log(
    state: State<'_, AppState>,
    item_id: Option<String>,
) -> Result<Vec<VaultAccessLog>, String> {
    let svc = service(&state);
    match item_id {
        Some(iid) => {
            let uuid = Uuid::parse_str(&iid).map_err(|e| format!("invalid item_id: {e}"))?;
            svc.get_access_log(uuid).await.map_err(|e| e.to_string())
        }
        None => svc.get_recent_access_logs(100).await.map_err(|e| e.to_string()),
    }
}

#[tauri::command]
pub async fn list_recent_vault_access(
    state: State<'_, AppState>,
    limit: Option<i64>,
) -> Result<Vec<VaultAccessLog>, String> {
    let svc = service(&state);
    svc.get_recent_access_logs(limit.unwrap_or(20))
        .await
        .map_err(|e| e.to_string())
}

fn parse_vault_item_type(s: &str) -> Result<VaultItemType, String> {
    match s {
        "password" => Ok(VaultItemType::Password),
        "note" => Ok(VaultItemType::Note),
        "key" => Ok(VaultItemType::Key),
        "secret" => Ok(VaultItemType::Secret),
        other => Err(format!("invalid vault item_type: {other}")),
    }
}
