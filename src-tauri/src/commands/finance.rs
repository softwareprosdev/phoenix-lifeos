use chrono::{DateTime, Datelike, TimeZone, Utc};
use phoenix_core::entities::{
    Account, AccountType, Category, CategoryType, Transaction, TransactionType,
};
use phoenix_core::value_objects::Currency;
use phoenix_db::repositories::*;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;

// ─── Payloads ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateAccountPayload {
    pub name: String,
    pub account_type: AccountType,
    pub institution: Option<String>,
    pub balance_cents: i64,
    pub currency: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateAccountPayload {
    pub id: String,
    pub name: String,
    pub account_type: AccountType,
    pub institution: Option<String>,
    pub balance_cents: i64,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct CreateTransactionPayload {
    pub account_id: String,
    pub amount_cents: i64,
    pub currency: Option<String>,
    pub transaction_type: TransactionType,
    pub category_id: Option<String>,
    pub merchant: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_recurring: Option<bool>,
}

#[derive(Deserialize)]
pub struct CreateCategoryPayload {
    pub name: String,
    pub category_type: CategoryType,
    pub parent_id: Option<String>,
}

#[derive(Deserialize)]
pub struct DateRangePayload {
    pub start: String,
    pub end: String,
}

// ─── Account Commands ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_accounts(state: State<'_, AppState>) -> Result<Vec<Account>, String> {
    let repo = SqlAccountRepository::new(state.pool.clone());
    repo.list_active().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_account(state: State<'_, AppState>, id: String) -> Result<Account, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlAccountRepository::new(state.pool.clone());
    repo.get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "account not found".to_string())
}

#[tauri::command]
pub async fn create_account(
    state: State<'_, AppState>,
    payload: CreateAccountPayload,
) -> Result<Account, String> {
    let repo = SqlAccountRepository::new(state.pool.clone());
    let currency = Currency::new(payload.currency.unwrap_or_else(|| "USD".into()))
        .map_err(|e| e.to_string())?;
    let now = Utc::now();
    let account = Account {
        id: Uuid::new_v4(),
        name: payload.name,
        account_type: payload.account_type,
        institution: payload.institution,
        balance_cents: payload.balance_cents,
        currency,
        is_active: true,
        created_at: now,
        updated_at: now,
    };
    repo.create(account).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_account(
    state: State<'_, AppState>,
    payload: UpdateAccountPayload,
) -> Result<Account, String> {
    let uuid = Uuid::parse_str(&payload.id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlAccountRepository::new(state.pool.clone());
    let existing = repo
        .get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "account not found".to_string())?;
    let now = Utc::now();
    let account = Account {
        id: uuid,
        name: payload.name,
        account_type: payload.account_type,
        institution: payload.institution,
        balance_cents: payload.balance_cents,
        currency: existing.currency,
        is_active: payload.is_active,
        created_at: existing.created_at,
        updated_at: now,
    };
    repo.update(account).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn deactivate_account(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlAccountRepository::new(state.pool.clone());
    repo.deactivate(uuid).await.map_err(|e| e.to_string())
}

// ─── Transaction Commands ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_transactions(
    state: State<'_, AppState>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Transaction>, String> {
    let repo = SqlTransactionRepository::new(state.pool.clone());
    repo.list_paginated(limit.unwrap_or(50), offset.unwrap_or(0))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_transaction(state: State<'_, AppState>, id: String) -> Result<Transaction, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlTransactionRepository::new(state.pool.clone());
    repo.get_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "transaction not found".to_string())
}

#[tauri::command]
pub async fn create_transaction(
    state: State<'_, AppState>,
    payload: CreateTransactionPayload,
) -> Result<Transaction, String> {
    let repo = SqlTransactionRepository::new(state.pool.clone());
    let account_uuid =
        Uuid::parse_str(&payload.account_id).map_err(|e| format!("invalid account_id: {e}"))?;
    let currency = Currency::new(payload.currency.unwrap_or_else(|| "USD".into()))
        .map_err(|e| e.to_string())?;
    let date = match &payload.date {
        Some(d) => DateTime::parse_from_rfc3339(d)
            .map_err(|e| format!("invalid date: {e}"))
            .map(|d| d.to_utc())?,
        None => Utc::now(),
    };
    let now = Utc::now();
    let tx = Transaction {
        id: Uuid::new_v4(),
        account_id: account_uuid,
        amount_cents: payload.amount_cents,
        currency,
        transaction_type: payload.transaction_type,
        category_id: payload
            .category_id
            .map(|c| Uuid::parse_str(&c))
            .transpose()
            .map_err(|e| format!("invalid category_id: {e}"))?,
        merchant: payload.merchant,
        description: payload.description,
        date,
        tags: payload.tags.unwrap_or_default(),
        is_recurring: payload.is_recurring.unwrap_or(false),
        created_at: now,
        updated_at: now,
        deleted_at: None,
    };
    repo.create(tx).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_transaction(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| format!("invalid id: {e}"))?;
    let repo = SqlTransactionRepository::new(state.pool.clone());
    repo.soft_delete(uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_transactions(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<Transaction>, String> {
    let repo = SqlTransactionRepository::new(state.pool.clone());
    repo.search(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_transactions_by_date_range(
    state: State<'_, AppState>,
    payload: DateRangePayload,
) -> Result<Vec<Transaction>, String> {
    let start = DateTime::parse_from_rfc3339(&payload.start)
        .map_err(|e| format!("invalid start date: {e}"))
        .map(|d| d.to_utc())?;
    let end = DateTime::parse_from_rfc3339(&payload.end)
        .map_err(|e| format!("invalid end date: {e}"))
        .map(|d| d.to_utc())?;
    let repo = SqlTransactionRepository::new(state.pool.clone());
    repo.list_by_date_range(start, end)
        .await
        .map_err(|e| e.to_string())
}

// ─── Summary / Analytics Commands ─────────────────────────────────────────────

#[derive(Serialize)]
pub struct NetWorthResponse {
    pub total_assets_cents: i64,
    pub total_liabilities_cents: i64,
    pub net_worth_cents: i64,
    pub currency: String,
}

#[tauri::command]
pub async fn get_net_worth(state: State<'_, AppState>) -> Result<NetWorthResponse, String> {
    let repo = SqlAccountRepository::new(state.pool.clone());
    let accounts = repo.list_active().await.map_err(|e| e.to_string())?;
    let mut assets = 0i64;
    let mut liabilities = 0i64;
    let currency = accounts
        .first()
        .map(|a| a.currency.code().to_string())
        .unwrap_or_else(|| "USD".into());
    for a in &accounts {
        match a.account_type {
            AccountType::Credit | AccountType::Loan => {
                liabilities += a.balance_cents.abs();
            }
            _ => {
                assets += a.balance_cents;
            }
        }
    }
    Ok(NetWorthResponse {
        total_assets_cents: assets,
        total_liabilities_cents: liabilities,
        net_worth_cents: assets - liabilities,
        currency,
    })
}

#[derive(Serialize)]
pub struct CashFlowResponse {
    pub total_income_cents: i64,
    pub total_expenses_cents: i64,
    pub net_cents: i64,
    pub currency: String,
    pub start_date: String,
    pub end_date: String,
}

#[tauri::command]
pub async fn get_cash_flow(
    state: State<'_, AppState>,
    payload: Option<DateRangePayload>,
) -> Result<CashFlowResponse, String> {
    let repo = SqlTransactionRepository::new(state.pool.clone());
    let now = Utc::now();
    let (start, end) = match payload {
        Some(p) => (
            DateTime::parse_from_rfc3339(&p.start)
                .map_err(|e| format!("invalid start date: {e}"))
                .map(|d| d.to_utc())?,
            DateTime::parse_from_rfc3339(&p.end)
                .map_err(|e| format!("invalid end date: {e}"))
                .map(|d| d.to_utc())?,
        ),
        None => {
            let start = Utc
                .with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0)
                .single()
                .unwrap_or(now);
            (start, now)
        }
    };
    let txs = repo
        .list_by_date_range(start, end)
        .await
        .map_err(|e| e.to_string())?;
    let mut income = 0i64;
    let mut expenses = 0i64;
    let currency = txs
        .first()
        .map(|t| t.currency.code().to_string())
        .unwrap_or_else(|| "USD".into());
    for tx in &txs {
        match tx.transaction_type {
            TransactionType::Income => income += tx.amount_cents,
            TransactionType::Expense => expenses += tx.amount_cents,
            TransactionType::Transfer => {}
        }
    }
    Ok(CashFlowResponse {
        total_income_cents: income,
        total_expenses_cents: expenses,
        net_cents: income - expenses,
        currency,
        start_date: start.to_rfc3339(),
        end_date: end.to_rfc3339(),
    })
}

#[derive(Serialize)]
pub struct SpendingByCategory {
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub total_cents: i64,
    pub transaction_count: usize,
}

#[tauri::command]
pub async fn get_spending_by_category(
    state: State<'_, AppState>,
    payload: Option<DateRangePayload>,
) -> Result<Vec<SpendingByCategory>, String> {
    let tx_repo = SqlTransactionRepository::new(state.pool.clone());
    let cat_repo = SqlCategoryRepository::new(state.pool.clone());
    let now = Utc::now();
    let (start, end) = match payload {
        Some(p) => (
            DateTime::parse_from_rfc3339(&p.start)
                .map_err(|e| format!("invalid start date: {e}"))
                .map(|d| d.to_utc())?,
            DateTime::parse_from_rfc3339(&p.end)
                .map_err(|e| format!("invalid end date: {e}"))
                .map(|d| d.to_utc())?,
        ),
        None => {
            let start = Utc
                .with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0)
                .single()
                .unwrap_or(now);
            (start, now)
        }
    };
    let txs = tx_repo
        .list_by_date_range(start, end)
        .await
        .map_err(|e| e.to_string())?;
    let categories = cat_repo
        .list_tree()
        .await
        .map_err(|e| e.to_string())?;
    let cat_map: std::collections::HashMap<Uuid, String> = categories
        .into_iter()
        .filter_map(|c| Some((c.id, c.name)))
        .collect();
    let mut spending: std::collections::HashMap<Option<Uuid>, (i64, usize)> =
        std::collections::HashMap::new();
    for tx in &txs {
        if tx.transaction_type != TransactionType::Expense {
            continue;
        }
        let entry = spending.entry(tx.category_id).or_insert((0i64, 0usize));
        entry.0 += tx.amount_cents;
        entry.1 += 1;
    }
    let mut result: Vec<SpendingByCategory> = spending
        .into_iter()
        .map(|(cat_id, (total, count))| SpendingByCategory {
            category_id: cat_id.map(|c| c.to_string()),
            category_name: cat_id.and_then(|c| cat_map.get(&c).cloned()),
            total_cents: total,
            transaction_count: count,
        })
        .collect();
    result.sort_by(|a, b| b.total_cents.cmp(&a.total_cents));
    Ok(result)
}

// ─── Category Commands ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_categories(state: State<'_, AppState>) -> Result<Vec<Category>, String> {
    let repo = SqlCategoryRepository::new(state.pool.clone());
    repo.list_tree().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_category(
    state: State<'_, AppState>,
    payload: CreateCategoryPayload,
) -> Result<Category, String> {
    let repo = SqlCategoryRepository::new(state.pool.clone());
    let category = Category {
        id: Uuid::new_v4(),
        name: payload.name,
        category_type: payload.category_type,
        parent_id: payload
            .parent_id
            .map(|p| Uuid::parse_str(&p))
            .transpose()
            .map_err(|e| format!("invalid parent_id: {e}"))?,
        created_at: Utc::now(),
    };
    repo.create(category).await.map_err(|e| e.to_string())
}
