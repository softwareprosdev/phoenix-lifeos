use chrono::{DateTime, Utc};
use phoenix_core::entities::CreditReport;
use phoenix_db::repositories::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;

// ─── Payloads ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateCreditReportPayload {
    pub score: i32,
    pub score_model: Option<String>,
    pub bureau: String,
    pub utilization_rate: f64,
    pub on_time_rate: f64,
    pub derogatory_marks: Option<i32>,
    pub total_accounts: Option<i32>,
    pub open_accounts: Option<i32>,
    pub hard_inquiries_12m: Option<i32>,
    pub report_date: Option<String>,
}

// ─── Response Types ───────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct CreditScoreHistoryPoint {
    pub score: i32,
    pub bureau: String,
    pub report_date: String,
}

// ─── Commands ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_credit_reports(
    state: State<'_, AppState>,
    start: Option<String>,
    end: Option<String>,
) -> Result<Vec<CreditReport>, String> {
    let repo = SqlCreditReportRepository::new(state.pool.clone());
    match (start, end) {
        (Some(s), Some(e)) => {
            let start_date = DateTime::parse_from_rfc3339(&s)
                .map_err(|e| format!("invalid start date: {e}"))
                .map(|d| d.to_utc())?;
            let end_date = DateTime::parse_from_rfc3339(&e)
                .map_err(|e| format!("invalid end date: {e}"))
                .map(|d| d.to_utc())?;
            repo.list_by_date_range(start_date, end_date)
                .await
                .map_err(|e| e.to_string())
        }
        _ => {
            let now = Utc::now();
            let start_date = now - chrono::Duration::days(365 * 2);
            repo.list_by_date_range(start_date, now)
                .await
                .map_err(|e| e.to_string())
        }
    }
}

#[tauri::command]
pub async fn get_latest_credit_report(
    state: State<'_, AppState>,
) -> Result<Option<CreditReport>, String> {
    let repo = SqlCreditReportRepository::new(state.pool.clone());
    repo.latest().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_credit_report(
    state: State<'_, AppState>,
    payload: CreateCreditReportPayload,
) -> Result<CreditReport, String> {
    let repo = SqlCreditReportRepository::new(state.pool.clone());
    let report_date = match &payload.report_date {
        Some(d) => DateTime::parse_from_rfc3339(d)
            .map_err(|e| format!("invalid report_date: {e}"))
            .map(|d| d.to_utc())?,
        None => Utc::now(),
    };
    let report = CreditReport {
        id: Uuid::new_v4(),
        score: payload.score,
        score_model: payload.score_model.unwrap_or_else(|| "FICO_8".into()),
        bureau: payload.bureau,
        utilization_rate: Decimal::try_from(payload.utilization_rate)
            .map_err(|e| format!("invalid utilization_rate: {e}"))?,
        on_time_rate: Decimal::try_from(payload.on_time_rate)
            .map_err(|e| format!("invalid on_time_rate: {e}"))?,
        derogatory_marks: payload.derogatory_marks.unwrap_or(0),
        total_accounts: payload.total_accounts.unwrap_or(0),
        open_accounts: payload.open_accounts.unwrap_or(0),
        hard_inquiries_12m: payload.hard_inquiries_12m.unwrap_or(0),
        report_date,
        created_at: Utc::now(),
    };
    repo.create(report).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_credit_score_history(
    state: State<'_, AppState>,
) -> Result<Vec<CreditScoreHistoryPoint>, String> {
    let repo = SqlCreditReportRepository::new(state.pool.clone());
    let now = Utc::now();
    let start = now - chrono::Duration::days(365 * 2);
    let reports = repo
        .list_by_date_range(start, now)
        .await
        .map_err(|e| e.to_string())?;
    let history: Vec<CreditScoreHistoryPoint> = reports
        .into_iter()
        .map(|r| CreditScoreHistoryPoint {
            score: r.score,
            bureau: r.bureau,
            report_date: r.report_date.to_rfc3339(),
        })
        .collect();
    Ok(history)
}
