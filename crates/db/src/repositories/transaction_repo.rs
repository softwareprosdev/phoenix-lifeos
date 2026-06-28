use async_trait::async_trait;
use chrono::{DateTime, Utc};
use phoenix_core::entities::{Transaction, TransactionType};
use phoenix_core::value_objects::Currency;
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn create(&self, tx: Transaction) -> anyhow::Result<Transaction>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Transaction>>;
    async fn list_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<Transaction>>;
    async fn list_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<Transaction>>;
    async fn list_by_category(&self, category_id: Uuid) -> anyhow::Result<Vec<Transaction>>;
    async fn search(&self, query: &str) -> anyhow::Result<Vec<Transaction>>;
    async fn soft_delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_paginated(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Transaction>>;
    async fn update(&self, tx: Transaction) -> anyhow::Result<Transaction>;
}

#[derive(Debug, Clone)]
pub struct SqlTransactionRepository {
    pool: DbPool,
}

impl SqlTransactionRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn map_transaction(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<Transaction> {
    let tx_type_str: String = row.get("transaction_type");
    let transaction_type = match tx_type_str.as_str() {
        "income" => TransactionType::Income,
        "expense" => TransactionType::Expense,
        "transfer" => TransactionType::Transfer,
        other => anyhow::bail!("invalid transaction_type: {other}"),
    };
    let tags_json: String = row.get("tags");
    let tags: Vec<String> = serde_json::from_str(&tags_json)?;
    let deleted_at: Option<String> = row.get("deleted_at");
    Ok(Transaction {
        id: row.get::<String, _>("id").parse()?,
        account_id: row.get::<String, _>("account_id").parse()?,
        amount_cents: row.get("amount_cents"),
        currency: Currency::new(row.get::<String, _>("currency"))?,
        transaction_type,
        category_id: row
            .get::<Option<String>, _>("category_id")
            .map(|s| s.parse())
            .transpose()?,
        merchant: row.get("merchant"),
        description: row.get("description"),
        date: row.get::<String, _>("date").parse()?,
        tags,
        is_recurring: row.get("is_recurring"),
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
        deleted_at: deleted_at.map(|s| s.parse()).transpose()?,
    })
}

fn tx_type_str(t: &TransactionType) -> &'static str {
    match t {
        TransactionType::Income => "income",
        TransactionType::Expense => "expense",
        TransactionType::Transfer => "transfer",
    }
}

#[async_trait]
impl TransactionRepository for SqlTransactionRepository {
    async fn create(&self, tx: Transaction) -> anyhow::Result<Transaction> {
        let tags_json = serde_json::to_string(&tx.tags)?;
        let result = sqlx::query(
            "INSERT INTO transactions (id, account_id, amount_cents, currency, transaction_type, category_id, merchant, description, date, tags, is_recurring, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(tx.id.to_string())
        .bind(tx.account_id.to_string())
        .bind(tx.amount_cents)
        .bind(tx.currency.code())
        .bind(tx_type_str(&tx.transaction_type))
        .bind(tx.category_id.map(|c| c.to_string()))
        .bind(&tx.merchant)
        .bind(&tx.description)
        .bind(tx.date.to_rfc3339())
        .bind(&tags_json)
        .bind(tx.is_recurring)
        .bind(tx.created_at.to_rfc3339())
        .bind(tx.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        let rowid = result.last_insert_rowid();
        if let Some(desc) = &tx.description {
            sqlx::query("INSERT INTO transactions_fts (rowid, description, merchant) VALUES (?, ?, ?)")
                .bind(rowid as i64)
                .bind(desc)
                .bind(&tx.merchant)
                .execute(&self.pool)
                .await
                .ok();
        }

        self.get_by_id(tx.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created transaction not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Transaction>> {
        let row = sqlx::query(
            "SELECT * FROM transactions WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;
        row.as_ref().map(map_transaction).transpose()
    }

    async fn list_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<Transaction>> {
        let rows = sqlx::query(
            "SELECT * FROM transactions WHERE account_id = ? AND deleted_at IS NULL ORDER BY date DESC",
        )
        .bind(account_id.to_string())
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_transaction).collect()
    }

    async fn list_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<Transaction>> {
        let rows = sqlx::query(
            "SELECT * FROM transactions WHERE date >= ? AND date <= ? AND deleted_at IS NULL ORDER BY date",
        )
        .bind(start.to_rfc3339())
        .bind(end.to_rfc3339())
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_transaction).collect()
    }

    async fn list_by_category(&self, category_id: Uuid) -> anyhow::Result<Vec<Transaction>> {
        let rows = sqlx::query(
            "SELECT * FROM transactions WHERE category_id = ? AND deleted_at IS NULL ORDER BY date DESC",
        )
        .bind(category_id.to_string())
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_transaction).collect()
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<Transaction>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }
        let like_pattern = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let rows = sqlx::query(
            "SELECT * FROM transactions WHERE deleted_at IS NULL AND (description LIKE ? ESCAPE '\\' OR merchant LIKE ? ESCAPE '\\') ORDER BY date DESC",
        )
        .bind(&like_pattern)
        .bind(&like_pattern)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_transaction).collect()
    }

    async fn soft_delete(&self, id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE transactions SET deleted_at = ?, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_paginated(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Transaction>> {
        let rows = sqlx::query(
            "SELECT * FROM transactions WHERE deleted_at IS NULL ORDER BY date DESC LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_transaction).collect()
    }

    async fn update(&self, tx: Transaction) -> anyhow::Result<Transaction> {
        let now = Utc::now().to_rfc3339();
        let tags_json = serde_json::to_string(&tx.tags)?;
        sqlx::query(
            "UPDATE transactions SET account_id = ?, amount_cents = ?, currency = ?, transaction_type = ?, category_id = ?, merchant = ?, description = ?, date = ?, tags = ?, is_recurring = ?, updated_at = ? WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(tx.account_id.to_string())
        .bind(tx.amount_cents)
        .bind(tx.currency.code())
        .bind(tx_type_str(&tx.transaction_type))
        .bind(tx.category_id.map(|c| c.to_string()))
        .bind(&tx.merchant)
        .bind(&tx.description)
        .bind(tx.date.to_rfc3339())
        .bind(&tags_json)
        .bind(tx.is_recurring)
        .bind(&now)
        .bind(tx.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(tx.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated transaction not found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phoenix_core::value_objects::Currency;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query("PRAGMA foreign_keys = OFF")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY, name TEXT NOT NULL,
                account_type TEXT NOT NULL, institution TEXT,
                balance_cents INTEGER NOT NULL DEFAULT 0,
                currency TEXT NOT NULL DEFAULT 'USD',
                is_active INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY, account_id TEXT NOT NULL,
                amount_cents INTEGER NOT NULL, currency TEXT NOT NULL DEFAULT 'USD',
                transaction_type TEXT NOT NULL, category_id TEXT,
                merchant TEXT, description TEXT, date TEXT NOT NULL,
                tags TEXT NOT NULL DEFAULT '[]', is_recurring INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE VIRTUAL TABLE IF NOT EXISTS transactions_fts USING fts5(description, merchant, content=transactions, content_rowid=rowid)",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    fn test_tx(account_id: Uuid) -> Transaction {
        Transaction {
            id: Uuid::new_v4(),
            account_id,
            amount_cents: 5000,
            currency: Currency::usd(),
            transaction_type: TransactionType::Expense,
            category_id: None,
            merchant: Some("Test Store".into()),
            description: Some("Test purchase".into()),
            date: Utc::now(),
            tags: vec!["test".into(), "shopping".into()],
            is_recurring: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let pool = create_test_db().await;
        let repo = SqlTransactionRepository::new(pool);
        let aid = Uuid::new_v4();
        let tx = test_tx(aid);
        let created = repo.create(tx).await.unwrap();
        assert_eq!(created.merchant.as_deref(), Some("Test Store"));
        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_some());
    }

    #[tokio::test]
    async fn test_soft_delete() {
        let pool = create_test_db().await;
        let repo = SqlTransactionRepository::new(pool);
        let aid = Uuid::new_v4();
        let tx = test_tx(aid);
        let created = repo.create(tx).await.unwrap();
        repo.soft_delete(created.id).await.unwrap();
        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_none());
    }

    #[tokio::test]
    async fn test_pagination() {
        let pool = create_test_db().await;
        let repo = SqlTransactionRepository::new(pool);
        let aid = Uuid::new_v4();
        for i in 0..5 {
            let mut tx = test_tx(aid);
            tx.id = Uuid::new_v4();
            tx.amount_cents = (i + 1) * 1000;
            repo.create(tx).await.unwrap();
        }
        let page = repo.list_paginated(2, 0).await.unwrap();
        assert_eq!(page.len(), 2);
    }
}
