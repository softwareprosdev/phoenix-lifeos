use chrono::{DateTime, Utc};
use phoenix_core::entities::{Debt, DebtType};
use phoenix_core::value_objects::{Currency, Percentage};
use phoenix_db::repositories::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;

// ─── Payloads ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateDebtPayload {
    pub name: String,
    pub debt_type: DebtType,
    pub principal_cents: i64,
    pub remaining_balance_cents: i64,
    pub interest_rate_pct: f64,
    pub minimum_payment_cents: i64,
    pub currency: Option<String>,
    pub due_date: Option<String>,
    pub creditor: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateDebtPayload {
    pub id: String,
    pub name: String,
    pub debt_type: DebtType,
    pub principal_cents: i64,
    pub remaining_balance_cents: i64,
    pub interest_rate_pct: f64,
    pub minimum_payment_cents: i64,
    pub due_date: Option<String>,
    pub creditor: Option<String>,
}

#[derive(Deserialize)]
pub struct DebtPaymentPayload {
    pub debt_id: String,
    pub amount_cents: i64,
}

// ─── Response Types ───────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct PayoffMonth {
    pub month: i32,
    pub payment_cents: i64,
    pub interest_cents: i64,
    pub principal_cents: i64,
    pub remaining_balance_cents: i64,
}

#[derive(Serialize)]
pub struct PayoffScheduleResponse {
    pub debt: Debt,
    pub months: Vec<PayoffMonth>,
    pub total_payments_cents: i64,
    pub total_interest_cents: i64,
    pub months_to_payoff: i32,
}

#[derive(Serialize)]
pub struct DebtSummaryResponse {
    pub total_debt_cents: i64,
    pub total_monthly_minimum_cents: i64,
    pub total_monthly_interest_cents: i64,
    pub debt_count: usize,
    pub currency: String,
}

// ─── Debt Commands ────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_debts(state: State<'_, AppState>) -> Result<Vec<Debt>, String> {
    let repo = SqlDebtRepository::new(state.pool.clone());
    repo.list_active().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_debt(state: State<'_, AppState>, id: String) -> Result<Debt, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlDebtRepository::new(state.pool.clone());
    repo.get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "debt not found".to_string())
}

#[tauri::command]
pub async fn create_debt(
    state: State<'_, AppState>,
    payload: CreateDebtPayload,
) -> Result<Debt, String> {
    let repo = SqlDebtRepository::new(state.pool.clone());
    let currency = Currency::new(payload.currency.unwrap_or_else(|| "USD".into()))
        .map_err(|e| e.to_string())?;
    let interest_rate =
        Percentage::new(Decimal::try_from(payload.interest_rate_pct).map_err(|e| format!("invalid interest rate: {e}"))?)
            .map_err(|e| e.to_string())?;
    let now = Utc::now();
    let debt = Debt {
        id: Uuid::new_v4(),
        name: payload.name,
        debt_type: payload.debt_type,
        principal_cents: payload.principal_cents,
        remaining_balance_cents: payload.remaining_balance_cents,
        interest_rate,
        minimum_payment_cents: payload.minimum_payment_cents,
        currency,
        due_date: payload
            .due_date
            .map(|d| DateTime::parse_from_rfc3339(&d))
            .transpose()
            .map_err(|e| format!("invalid due_date: {e}"))?
            .map(|d| d.to_utc()),
        creditor: payload.creditor,
        created_at: now,
        updated_at: now,
    };
    repo.create(debt).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_debt(
    state: State<'_, AppState>,
    payload: UpdateDebtPayload,
) -> Result<Debt, String> {
    let uuid = Uuid::parse_str(&payload.id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlDebtRepository::new(state.pool.clone());
    let existing = repo
        .get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "debt not found".to_string())?;
    let interest_rate =
        Percentage::new(Decimal::try_from(payload.interest_rate_pct).map_err(|e| format!("invalid interest rate: {e}"))?)
            .map_err(|e| e.to_string())?;
    let debt = Debt {
        id: uuid,
        name: payload.name,
        debt_type: payload.debt_type,
        principal_cents: payload.principal_cents,
        remaining_balance_cents: payload.remaining_balance_cents,
        interest_rate,
        minimum_payment_cents: payload.minimum_payment_cents,
        currency: existing.currency,
        due_date: payload
            .due_date
            .map(|d| DateTime::parse_from_rfc3339(&d))
            .transpose()
            .map_err(|e| format!("invalid due_date: {e}"))?
            .map(|d| d.to_utc()),
        creditor: payload.creditor,
        created_at: existing.created_at,
        updated_at: Utc::now(),
    };
    repo.update(debt).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn make_debt_payment(
    state: State<'_, AppState>,
    payload: DebtPaymentPayload,
) -> Result<Debt, String> {
    let uuid = Uuid::parse_str(&payload.debt_id).map_err(|e| format!("invalid debt_id: {e}"))?;
    let repo = SqlDebtRepository::new(state.pool.clone());
    let mut debt = repo
        .get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "debt not found".to_string())?;
    if payload.amount_cents <= 0 {
        return Err("payment amount must be positive".to_string());
    }
    debt.remaining_balance_cents = (debt.remaining_balance_cents - payload.amount_cents).max(0);
    debt.updated_at = Utc::now();
    repo.update(debt).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_payoff_schedule(
    state: State<'_, AppState>,
    debt_id: String,
    monthly_payment_cents: Option<i64>,
) -> Result<PayoffScheduleResponse, String> {
    let uuid = Uuid::parse_str(&debt_id).map_err(|e| format!("invalid debt_id: {e}"))?;
    let repo = SqlDebtRepository::new(state.pool.clone());
    let debt = repo
        .get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "debt not found".to_string())?;

    let payment = monthly_payment_cents.unwrap_or(debt.minimum_payment_cents);
    if payment <= 0 {
        return Err("monthly payment must be positive".to_string());
    }

    let monthly_rate = debt.interest_rate.as_rate() / Decimal::from(12);
    let mut balance = Decimal::from(debt.remaining_balance_cents);
    let payment_dec = Decimal::from(payment);
    let mut months = Vec::new();
    let mut total_interest = Decimal::ZERO;
    let mut month_count = 0i32;

    while balance > Decimal::ZERO && month_count < 1200 {
        month_count += 1;
        let interest = (balance * monthly_rate).round();
        let principal = if payment_dec >= balance + interest {
            balance + interest
        } else {
            payment_dec
        };
        let actual_interest = interest.min(principal);
        let actual_principal = principal - actual_interest;
        balance = (balance - actual_principal).max(Decimal::ZERO);
        total_interest += actual_interest;

        months.push(PayoffMonth {
            month: month_count,
            payment_cents: principal.try_into().unwrap_or(0),
            interest_cents: actual_interest.try_into().unwrap_or(0),
            principal_cents: actual_principal.try_into().unwrap_or(0),
            remaining_balance_cents: balance.try_into().unwrap_or(0),
        });

        if balance <= Decimal::ZERO {
            break;
        }
    }

    Ok(PayoffScheduleResponse {
        total_payments_cents: months.iter().map(|m| m.payment_cents).sum(),
        total_interest_cents: months.iter().map(|m| m.interest_cents).sum(),
        months_to_payoff: month_count,
        months,
        debt,
    })
}

#[tauri::command]
pub async fn get_debt_summary(state: State<'_, AppState>) -> Result<DebtSummaryResponse, String> {
    let repo = SqlDebtRepository::new(state.pool.clone());
    let debts = repo.list_active().await.map_err(|e| e.to_string())?;
    let currency = debts
        .first()
        .map(|d| d.currency.code().to_string())
        .unwrap_or_else(|| "USD".into());
    let total_debt_cents: i64 = debts.iter().map(|d| d.remaining_balance_cents).sum();
    let total_minimum: i64 = debts.iter().map(|d| d.minimum_payment_cents).sum();
    let total_interest: i64 = debts.iter().map(|d| d.monthly_interest_cents()).sum();
    Ok(DebtSummaryResponse {
        total_debt_cents,
        total_monthly_minimum_cents: total_minimum,
        total_monthly_interest_cents: total_interest,
        debt_count: debts.len(),
        currency,
    })
}
