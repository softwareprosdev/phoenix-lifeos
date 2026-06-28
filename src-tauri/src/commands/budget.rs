use chrono::{DateTime, Utc};
use phoenix_core::entities::{Budget, BudgetItem, BudgetPeriod};
use phoenix_core::value_objects::Currency;
use phoenix_db::repositories::*;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;

// ─── Payloads ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateBudgetPayload {
    pub name: String,
    pub period: BudgetPeriod,
    pub total_limit_cents: i64,
    pub currency: Option<String>,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Deserialize)]
pub struct UpdateBudgetPayload {
    pub id: String,
    pub name: String,
    pub period: BudgetPeriod,
    pub total_limit_cents: i64,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Deserialize)]
pub struct CreateBudgetItemPayload {
    pub budget_id: String,
    pub category_id: Option<String>,
    pub limit_cents: i64,
    pub spent_cents: Option<i64>,
}

// ─── Budget Commands ──────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct BudgetProgressResponse {
    pub budget: Budget,
    pub items: Vec<BudgetItem>,
    pub total_spent_cents: i64,
    pub total_limit_cents: i64,
    pub remaining_cents: i64,
    pub progress_pct: f64,
}

#[tauri::command]
pub async fn list_budgets(state: State<'_, AppState>) -> Result<Vec<Budget>, String> {
    let repo = SqlBudgetRepository::new(state.pool.clone());
    let now = Utc::now();
    let end = now + chrono::Duration::days(365);
    let start = now - chrono::Duration::days(365);
    repo.list_by_date_range(start, end)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_budget(state: State<'_, AppState>, id: String) -> Result<Budget, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlBudgetRepository::new(state.pool.clone());
    repo.get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "budget not found".to_string())
}

#[tauri::command]
pub async fn create_budget(
    state: State<'_, AppState>,
    payload: CreateBudgetPayload,
) -> Result<Budget, String> {
    let repo = SqlBudgetRepository::new(state.pool.clone());
    let currency = Currency::new(payload.currency.unwrap_or_else(|| "USD".into()))
        .map_err(|e| e.to_string())?;
    let start_date = DateTime::parse_from_rfc3339(&payload.start_date)
        .map_err(|e| format!("invalid start_date: {e}"))
        .map(|d| d.to_utc())?;
    let end_date = DateTime::parse_from_rfc3339(&payload.end_date)
        .map_err(|e| format!("invalid end_date: {e}"))
        .map(|d| d.to_utc())?;
    let budget = Budget {
        id: Uuid::new_v4(),
        name: payload.name,
        period: payload.period,
        total_limit_cents: payload.total_limit_cents,
        currency,
        start_date,
        end_date,
        created_at: Utc::now(),
    };
    repo.create(budget).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_budget(
    state: State<'_, AppState>,
    payload: UpdateBudgetPayload,
) -> Result<Budget, String> {
    let uuid = Uuid::parse_str(&payload.id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlBudgetRepository::new(state.pool.clone());
    let existing = repo
        .get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "budget not found".to_string())?;
    let start_date = DateTime::parse_from_rfc3339(&payload.start_date)
        .map_err(|e| format!("invalid start_date: {e}"))
        .map(|d| d.to_utc())?;
    let end_date = DateTime::parse_from_rfc3339(&payload.end_date)
        .map_err(|e| format!("invalid end_date: {e}"))
        .map(|d| d.to_utc())?;
    let budget = Budget {
        id: uuid,
        name: payload.name,
        period: payload.period,
        total_limit_cents: payload.total_limit_cents,
        currency: existing.currency,
        start_date,
        end_date,
        created_at: existing.created_at,
    };
    repo.update(budget).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_budget_progress(
    state: State<'_, AppState>,
    id: String,
) -> Result<BudgetProgressResponse, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlBudgetRepository::new(state.pool.clone());
    let (budget, items) = repo
        .get_with_items(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "budget not found".to_string())?;
    let total_spent_cents: i64 = items.iter().map(|i| i.spent_cents).sum();
    let total_limit_cents = budget.total_limit_cents;
    let remaining_cents = (total_limit_cents - total_spent_cents).max(0);
    let progress_pct = if total_limit_cents > 0 {
        (total_spent_cents as f64 / total_limit_cents as f64 * 100.0).min(100.0)
    } else {
        0.0
    };
    Ok(BudgetProgressResponse {
        budget,
        items,
        total_spent_cents,
        total_limit_cents,
        remaining_cents,
        progress_pct,
    })
}

// ─── Budget Item Commands ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_budget_items(
    state: State<'_, AppState>,
    budget_id: String,
) -> Result<Vec<BudgetItem>, String> {
    let uuid = Uuid::parse_str(&budget_id).map_err(|e| format!("invalid budget_id: {e}"))?;
    let repo = SqlBudgetItemRepository::new(state.pool.clone());
    repo.list_by_budget(uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_budget_item(
    state: State<'_, AppState>,
    payload: CreateBudgetItemPayload,
) -> Result<BudgetItem, String> {
    let repo = SqlBudgetItemRepository::new(state.pool.clone());
    let budget_uuid =
        Uuid::parse_str(&payload.budget_id).map_err(|e| format!("invalid budget_id: {e}"))?;
    let item = BudgetItem {
        id: Uuid::new_v4(),
        budget_id: budget_uuid,
        category_id: payload
            .category_id
            .map(|c| Uuid::parse_str(&c))
            .transpose()
            .map_err(|e| format!("invalid category_id: {e}"))?,
        limit_cents: payload.limit_cents,
        spent_cents: payload.spent_cents.unwrap_or(0),
    };
    repo.create(item).await.map_err(|e| e.to_string())
}
