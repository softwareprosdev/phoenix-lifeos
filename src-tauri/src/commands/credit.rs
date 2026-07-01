use std::collections::HashMap;

use chrono::{DateTime, Utc};
use phoenix_core::entities::{
    BusinessCreditProfile, CreditAccount, CreditDerogatory, CreditInquiry, CreditReport,
    DisputeCase, DisputeLetter, DisputeLetterTemplate, DisputeTimelineEntry,
};
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

#[derive(Deserialize)]
pub struct CreateDisputeCasePayload {
    pub negative_item_id: Option<String>,
    pub dispute_type: String,
    pub bureau: String,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateDisputeCasePayload {
    pub status: Option<String>,
    pub status_basis: Option<String>,
    pub statute_basis: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct GenerateLetterPayload {
    pub case_id: String,
    pub template_id: String,
    pub variables: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct CreateCreditAccountPayload {
    pub name: String,
    pub issuer: String,
    pub account_type: String,
    pub credit_limit_cents: i64,
    pub balance_cents: i64,
    pub apr: String,
    pub payment_status: String,
    pub opened_date: Option<String>,
    pub closed_date: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateCreditInquiryPayload {
    pub bureau: String,
    pub creditor_name: String,
    pub inquiry_type: String,
    pub inquiry_date: String,
}

#[derive(Deserialize)]
pub struct CreateDerogatoryItemPayload {
    pub creditor_name: String,
    pub account_number_encrypted: Option<String>,
    pub item_type: String,
    pub amount_cents: i64,
    pub original_amount_cents: i64,
    pub status: String,
    pub opened_date: Option<String>,
    pub expected_removal_date: Option<String>,
    pub bureau: String,
}

#[derive(Deserialize)]
pub struct CreateBusinessProfilePayload {
    pub business_name: String,
    pub ein_encrypted: Option<String>,
    pub duns_number: Option<String>,
    pub paydex_score: Option<i32>,
    pub business_credit_score: Option<i32>,
}

#[derive(Deserialize)]
pub struct MarkLetterSentPayload {
    pub letter_id: String,
    pub tracking_number: Option<String>,
}

#[derive(Deserialize)]
pub struct AddTimelineEntryPayload {
    pub case_id: String,
    pub event_type: String,
    pub description: String,
}

// ─── Response Types ───────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct CreditScoreHistoryPoint {
    pub score: i32,
    pub bureau: String,
    pub report_date: String,
}

#[derive(Serialize)]
pub struct DisputeCaseResponse {
    pub id: String,
    pub negative_item_id: Option<String>,
    pub dispute_type: String,
    pub status: String,
    pub status_basis: Option<String>,
    pub bureau: String,
    pub statute_basis: Option<String>,
    pub notes: Option<String>,
    pub creditor_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct DisputeCaseDetailResponse {
    pub id: String,
    pub negative_item_id: Option<String>,
    pub dispute_type: String,
    pub status: String,
    pub status_basis: Option<String>,
    pub bureau: String,
    pub statute_basis: Option<String>,
    pub notes: Option<String>,
    pub timeline: Vec<DisputeTimelineEntry>,
    pub created_at: String,
    pub updated_at: String,
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

// ─── Dispute Case Commands ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_dispute_cases(
    state: State<'_, AppState>,
) -> Result<Vec<DisputeCaseResponse>, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let results = repo
        .list_dispute_cases_with_negative()
        .await
        .map_err(|e| e.to_string())?;
    Ok(results
        .into_iter()
        .map(|(case, creditor_name)| DisputeCaseResponse {
            id: case.id.to_string(),
            negative_item_id: case.negative_item_id.map(|id| id.to_string()),
            dispute_type: case.dispute_type,
            status: case.status,
            status_basis: case.status_basis,
            bureau: case.bureau,
            statute_basis: case.statute_basis,
            notes: case.notes,
            creditor_name,
            created_at: case.created_at.to_rfc3339(),
            updated_at: case.updated_at.to_rfc3339(),
        })
        .collect())
}

#[tauri::command]
pub async fn create_dispute_case(
    state: State<'_, AppState>,
    payload: CreateDisputeCasePayload,
) -> Result<DisputeCase, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let negative_item_id = payload
        .negative_item_id
        .as_ref()
        .and_then(|s| Uuid::parse_str(s).ok());
    let now = Utc::now();
    let case = DisputeCase {
        id: Uuid::new_v4(),
        negative_item_id,
        dispute_type: payload.dispute_type,
        status: "draft".into(),
        status_basis: None,
        bureau: payload.bureau,
        statute_basis: None,
        notes: payload.notes,
        created_at: now,
        updated_at: now,
    };
    let created = repo.create_dispute_case(case).await.map_err(|e| e.to_string())?;

    repo.create_timeline_entry(DisputeTimelineEntry {
        id: Uuid::new_v4(),
        dispute_case_id: created.id,
        event_type: "dispute_filed".into(),
        description: "Dispute case created".into(),
        metadata: None,
        created_at: Utc::now(),
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(created)
}

#[tauri::command]
pub async fn get_dispute_case(
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<DisputeCaseDetailResponse>, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let case_id = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let case = repo.get_dispute_case(case_id).await.map_err(|e| e.to_string())?;
    match case {
        Some(c) => {
            let timeline = repo
                .list_timeline_by_case(case_id)
                .await
                .map_err(|e| e.to_string())?;
            Ok(Some(DisputeCaseDetailResponse {
                id: c.id.to_string(),
                negative_item_id: c.negative_item_id.map(|id| id.to_string()),
                dispute_type: c.dispute_type,
                status: c.status,
                status_basis: c.status_basis,
                bureau: c.bureau,
                statute_basis: c.statute_basis,
                notes: c.notes,
                timeline,
                created_at: c.created_at.to_rfc3339(),
                updated_at: c.updated_at.to_rfc3339(),
            }))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn update_dispute_case(
    state: State<'_, AppState>,
    id: String,
    payload: UpdateDisputeCasePayload,
) -> Result<DisputeCase, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let case_id = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let mut case = repo
        .get_dispute_case(case_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "dispute case not found".to_string())?;

    if let Some(status) = payload.status {
        case.status = status;
    }
    if let Some(basis) = payload.status_basis {
        case.status_basis = Some(basis);
    }
    if let Some(basis) = payload.statute_basis {
        case.statute_basis = Some(basis);
    }
    if let Some(notes) = payload.notes {
        case.notes = Some(notes);
    }
    case.updated_at = Utc::now();

    repo.update_dispute_case(case).await.map_err(|e| e.to_string())
}

// ─── Dispute Letter Commands ──────────────────────────────────────────────────

#[tauri::command]
pub async fn list_dispute_letters(
    state: State<'_, AppState>,
    case_id: String,
) -> Result<Vec<DisputeLetter>, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let id = Uuid::parse_str(&case_id).map_err(|e| format!("invalid case_id: {e}"))?;
    repo.list_dispute_letters_by_case(id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_letter(
    state: State<'_, AppState>,
    payload: GenerateLetterPayload,
) -> Result<DisputeLetter, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let case_id = Uuid::parse_str(&payload.case_id).map_err(|e| format!("invalid case_id: {e}"))?;

    let template = repo
        .get_template_by_id(&payload.template_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("template not found: {}", payload.template_id))?;

    let mut body = template.body_template.clone();
    for (key, value) in &payload.variables {
        body = body.replace(&format!("{{{{{}}}}}", key), value);
    }

    let now = Utc::now();
    let letter = DisputeLetter {
        id: Uuid::new_v4(),
        dispute_case_id: case_id,
        template_id: Some(payload.template_id),
        recipient_type: template.recipient_type,
        recipient_name: template.name,
        recipient_address: None,
        generated_content_encrypted: Some(body),
        tracking_number: None,
        sent_date: None,
        response_received_date: None,
        response_summary: None,
        status: "draft".into(),
        created_at: now,
        updated_at: now,
    };

    let created = repo.create_dispute_letter(letter).await.map_err(|e| e.to_string())?;

    repo.create_timeline_entry(DisputeTimelineEntry {
        id: Uuid::new_v4(),
        dispute_case_id: case_id,
        event_type: "letter_sent".into(),
        description: format!("Letter generated using template {}", template.template_code),
        metadata: None,
        created_at: Utc::now(),
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(created)
}

#[tauri::command]
pub async fn list_letter_templates(
    state: State<'_, AppState>,
) -> Result<Vec<DisputeLetterTemplate>, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    repo.list_templates().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_letter_template_by_code(
    state: State<'_, AppState>,
    code: String,
) -> Result<Option<DisputeLetterTemplate>, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    repo.get_template_by_code(&code)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mark_letter_sent(
    state: State<'_, AppState>,
    payload: MarkLetterSentPayload,
) -> Result<DisputeLetter, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let letter_id = Uuid::parse_str(&payload.letter_id).map_err(|e| format!("invalid letter_id: {e}"))?;
    let mut letter = repo
        .get_dispute_letter(letter_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "dispute letter not found".to_string())?;

    letter.status = "sent".into();
    letter.tracking_number = payload.tracking_number;
    letter.sent_date = Some(Utc::now());
    letter.updated_at = Utc::now();

    repo.update_dispute_letter(letter).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_timeline_entry(
    state: State<'_, AppState>,
    payload: AddTimelineEntryPayload,
) -> Result<DisputeTimelineEntry, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let case_id = Uuid::parse_str(&payload.case_id).map_err(|e| format!("invalid case_id: {e}"))?;
    let entry = DisputeTimelineEntry {
        id: Uuid::new_v4(),
        dispute_case_id: case_id,
        event_type: payload.event_type,
        description: payload.description,
        metadata: None,
        created_at: Utc::now(),
    };
    repo.create_timeline_entry(entry).await.map_err(|e| e.to_string())
}

// ─── Credit Account Commands ──────────────────────────────────────────────────

#[tauri::command]
pub async fn get_credit_accounts(
    state: State<'_, AppState>,
) -> Result<Vec<CreditAccount>, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    repo.list_credit_accounts().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_credit_account(
    state: State<'_, AppState>,
    payload: CreateCreditAccountPayload,
) -> Result<CreditAccount, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let opened_date = payload
        .opened_date
        .as_ref()
        .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
        .map(|d| d.to_utc());
    let closed_date = payload
        .closed_date
        .as_ref()
        .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
        .map(|d| d.to_utc());
    let now = Utc::now();
    let account = CreditAccount {
        id: Uuid::new_v4(),
        name: payload.name,
        issuer: payload.issuer,
        account_type: payload.account_type,
        credit_limit_cents: payload.credit_limit_cents,
        balance_cents: payload.balance_cents,
        apr: payload.apr,
        payment_status: payload.payment_status,
        opened_date,
        closed_date,
        is_active: true,
        created_at: now,
        updated_at: now,
    };
    repo.create_credit_account(account).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_credit_account(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let account_id = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    repo.delete_credit_account(account_id)
        .await
        .map_err(|e| e.to_string())
}

// ─── Credit Inquiry Commands ──────────────────────────────────────────────────

#[tauri::command]
pub async fn list_credit_inquiries(
    state: State<'_, AppState>,
) -> Result<Vec<CreditInquiry>, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    repo.list_credit_inquiries().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_credit_inquiry(
    state: State<'_, AppState>,
    payload: CreateCreditInquiryPayload,
) -> Result<CreditInquiry, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let inquiry_date = DateTime::parse_from_rfc3339(&payload.inquiry_date)
        .map_err(|e| format!("invalid inquiry_date: {e}"))
        .map(|d| d.to_utc())?;
    let inquiry = CreditInquiry {
        id: Uuid::new_v4(),
        bureau: payload.bureau,
        creditor_name: payload.creditor_name,
        inquiry_type: payload.inquiry_type,
        inquiry_date,
        created_at: Utc::now(),
    };
    repo.create_credit_inquiry(inquiry).await.map_err(|e| e.to_string())
}

// ─── Derogatory Item Commands ─────────────────────────────────────────────────

#[tauri::command]
pub async fn list_derogatory_items(
    state: State<'_, AppState>,
) -> Result<Vec<CreditDerogatory>, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    repo.list_credit_derogatory().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_derogatory_item(
    state: State<'_, AppState>,
    payload: CreateDerogatoryItemPayload,
) -> Result<CreditDerogatory, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let opened_date = payload
        .opened_date
        .as_ref()
        .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
        .map(|d| d.to_utc());
    let expected_removal_date = payload
        .expected_removal_date
        .as_ref()
        .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
        .map(|d| d.to_utc());
    let now = Utc::now();
    let item = CreditDerogatory {
        id: Uuid::new_v4(),
        creditor_name: payload.creditor_name,
        account_number_encrypted: payload.account_number_encrypted,
        item_type: payload.item_type,
        amount_cents: payload.amount_cents,
        original_amount_cents: payload.original_amount_cents,
        status: payload.status,
        opened_date,
        expected_removal_date,
        is_disputed: false,
        bureau: payload.bureau,
        created_at: now,
        updated_at: now,
    };
    repo.create_credit_derogatory(item).await.map_err(|e| e.to_string())
}

// ─── Business Credit Commands ─────────────────────────────────────────────────

#[tauri::command]
pub async fn get_business_credit_profiles(
    state: State<'_, AppState>,
) -> Result<Vec<BusinessCreditProfile>, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    repo.list_business_profiles().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_business_credit_profile(
    state: State<'_, AppState>,
    payload: CreateBusinessProfilePayload,
) -> Result<BusinessCreditProfile, String> {
    let repo = SqlCreditRepairRepository::new(state.pool.clone());
    let now = Utc::now();
    let profile = BusinessCreditProfile {
        id: Uuid::new_v4(),
        business_name: payload.business_name,
        ein_encrypted: payload.ein_encrypted,
        duns_number: payload.duns_number,
        paydex_score: payload.paydex_score,
        business_credit_score: payload.business_credit_score,
        created_at: now,
        updated_at: now,
    };
    repo.create_business_profile(profile).await.map_err(|e| e.to_string())
}

// ─── Delete Helper Commands ──────────────────────────────────────────────────

#[tauri::command]
pub async fn delete_credit_report(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let repo = SqlCreditReportRepository::new(state.pool.clone());
    let report_id = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    repo.delete(report_id).await.map_err(|e| e.to_string())
}
