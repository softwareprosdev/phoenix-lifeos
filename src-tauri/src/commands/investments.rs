use chrono::Utc;
use phoenix_core::entities::{AssetType, Investment};
use phoenix_core::value_objects::Currency;
use phoenix_db::repositories::*;
use rust_decimal::Decimal;
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;

// ─── Payloads ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateInvestmentPayload {
    pub account_id: String,
    pub asset_type: AssetType,
    pub symbol: Option<String>,
    pub name: String,
    pub quantity: f64,
    pub cost_basis_cents: i64,
    pub current_value_cents: i64,
    pub currency: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateInvestmentPayload {
    pub id: String,
    pub account_id: Option<String>,
    pub asset_type: Option<AssetType>,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub quantity: Option<f64>,
    pub cost_basis_cents: Option<i64>,
    pub current_value_cents: Option<i64>,
}

// ─── Commands ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_investments(
    state: State<'_, AppState>,
    account_id: Option<String>,
) -> Result<Vec<Investment>, String> {
    let repo = SqlInvestmentRepository::new(state.pool.clone());
    match account_id {
        Some(aid) => {
            let uuid = Uuid::parse_str(&aid).map_err(|e| format!("invalid account_id: {e}"))?;
            repo.list_by_account(uuid)
                .await
                .map_err(|e| e.to_string())
        }
        None => {
            let empty = Vec::new();
            Ok(empty)
        }
    }
}

#[tauri::command]
pub async fn create_investment(
    state: State<'_, AppState>,
    payload: CreateInvestmentPayload,
) -> Result<Investment, String> {
    let repo = SqlInvestmentRepository::new(state.pool.clone());
    let account_uuid =
        Uuid::parse_str(&payload.account_id).map_err(|e| format!("invalid account_id: {e}"))?;
    let currency = Currency::new(payload.currency.unwrap_or_else(|| "USD".into()))
        .map_err(|e| e.to_string())?;
    let now = Utc::now();
    let investment = Investment {
        id: Uuid::new_v4(),
        account_id: account_uuid,
        asset_type: payload.asset_type,
        symbol: payload.symbol,
        name: payload.name,
        quantity: Decimal::try_from(payload.quantity)
            .map_err(|e| format!("invalid quantity: {e}"))?,
        cost_basis_cents: payload.cost_basis_cents,
        current_value_cents: payload.current_value_cents,
        currency,
        created_at: now,
        updated_at: now,
    };
    repo.create(investment).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_investment(
    state: State<'_, AppState>,
    payload: UpdateInvestmentPayload,
) -> Result<Investment, String> {
    let uuid = Uuid::parse_str(&payload.id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlInvestmentRepository::new(state.pool.clone());
    let mut investment = repo
        .get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "investment not found".to_string())?;

    if let Some(account_id) = payload.account_id {
        investment.account_id =
            Uuid::parse_str(&account_id).map_err(|e| format!("invalid account_id: {e}"))?;
    }
    if let Some(asset_type) = payload.asset_type {
        investment.asset_type = asset_type;
    }
    if let Some(symbol) = payload.symbol {
        investment.symbol = Some(symbol);
    }
    if let Some(name) = payload.name {
        investment.name = name;
    }
    if let Some(quantity) = payload.quantity {
        investment.quantity =
            Decimal::try_from(quantity).map_err(|e| format!("invalid quantity: {e}"))?;
    }
    if let Some(cost_basis_cents) = payload.cost_basis_cents {
        investment.cost_basis_cents = cost_basis_cents;
    }
    if let Some(current_value_cents) = payload.current_value_cents {
        investment.current_value_cents = current_value_cents;
    }
    investment.updated_at = Utc::now();

    repo.update(investment).await.map_err(|e| e.to_string())
}
