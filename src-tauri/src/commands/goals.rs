use chrono::{DateTime, Utc};
use phoenix_core::entities::{Goal, GoalType};
use phoenix_core::value_objects::Currency;
use phoenix_db::repositories::*;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;

// ─── Payloads ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateGoalPayload {
    pub name: String,
    pub goal_type: GoalType,
    pub target_amount_cents: i64,
    pub current_amount_cents: Option<i64>,
    pub monthly_contribution_cents: Option<i64>,
    pub currency: Option<String>,
    pub deadline: Option<String>,
    pub linked_account_id: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateGoalPayload {
    pub id: String,
    pub name: String,
    pub goal_type: GoalType,
    pub target_amount_cents: i64,
    pub current_amount_cents: i64,
    pub monthly_contribution_cents: Option<i64>,
    pub deadline: Option<String>,
    pub linked_account_id: Option<String>,
}

// ─── Response Types ───────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct GoalProgressResponse {
    pub goal: Goal,
    pub progress_pct: f64,
    pub remaining_cents: i64,
    pub is_achieved: bool,
}

// ─── Commands ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_goals(state: State<'_, AppState>) -> Result<Vec<Goal>, String> {
    let repo = SqlGoalRepository::new(state.pool.clone());
    repo.list_active().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_goal(state: State<'_, AppState>, id: String) -> Result<Goal, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlGoalRepository::new(state.pool.clone());
    repo.get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "goal not found".to_string())
}

#[tauri::command]
pub async fn create_goal(
    state: State<'_, AppState>,
    payload: CreateGoalPayload,
) -> Result<Goal, String> {
    let repo = SqlGoalRepository::new(state.pool.clone());
    let currency = Currency::new(payload.currency.unwrap_or_else(|| "USD".into()))
        .map_err(|e| e.to_string())?;
    let now = Utc::now();
    let goal = Goal {
        id: Uuid::new_v4(),
        name: payload.name,
        goal_type: payload.goal_type,
        target_amount_cents: payload.target_amount_cents,
        current_amount_cents: payload.current_amount_cents.unwrap_or(0),
        monthly_contribution_cents: payload.monthly_contribution_cents.unwrap_or(0),
        currency,
        deadline: payload
            .deadline
            .map(|d| DateTime::parse_from_rfc3339(&d))
            .transpose()
            .map_err(|e| format!("invalid deadline: {e}"))?
            .map(|d| d.to_utc()),
        linked_account_id: payload
            .linked_account_id
            .map(|a| Uuid::parse_str(&a))
            .transpose()
            .map_err(|e| format!("invalid linked_account_id: {e}"))?,
        created_at: now,
        updated_at: now,
    };
    repo.create(goal).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_goal(
    state: State<'_, AppState>,
    payload: UpdateGoalPayload,
) -> Result<Goal, String> {
    let uuid = Uuid::parse_str(&payload.id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlGoalRepository::new(state.pool.clone());
    let existing = repo
        .get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "goal not found".to_string())?;
    let now = Utc::now();
    let goal = Goal {
        id: uuid,
        name: payload.name,
        goal_type: payload.goal_type,
        target_amount_cents: payload.target_amount_cents,
        current_amount_cents: payload.current_amount_cents,
        monthly_contribution_cents: payload.monthly_contribution_cents.unwrap_or(existing.monthly_contribution_cents),
        currency: existing.currency,
        deadline: payload
            .deadline
            .map(|d| DateTime::parse_from_rfc3339(&d))
            .transpose()
            .map_err(|e| format!("invalid deadline: {e}"))?
            .map(|d| d.to_utc()),
        linked_account_id: payload
            .linked_account_id
            .map(|a| Uuid::parse_str(&a))
            .transpose()
            .map_err(|e| format!("invalid linked_account_id: {e}"))?,
        created_at: existing.created_at,
        updated_at: now,
    };
    repo.update(goal).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_goal_progress(
    state: State<'_, AppState>,
    id: String,
) -> Result<GoalProgressResponse, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlGoalRepository::new(state.pool.clone());
    let goal = repo
        .get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "goal not found".to_string())?;
    Ok(GoalProgressResponse {
        progress_pct: goal.progress_percentage(),
        remaining_cents: goal.remaining_amount_cents(),
        is_achieved: goal.is_achieved(),
        goal,
    })
}
