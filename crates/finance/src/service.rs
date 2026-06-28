use chrono::{DateTime, Utc};
use phoenix_core::entities::{
    Account, AccountType, Category, CategoryType, Transaction, TransactionType,
};
use phoenix_core::event_bus::{DomainEvent, EventBus};
use phoenix_core::value_objects::Currency;
use phoenix_db::repositories::{
    account_repo::{AccountRepository as _, SqlAccountRepository},
    category_repo::{CategoryRepository as _, SqlCategoryRepository},
    transaction_repo::{TransactionRepository as _, SqlTransactionRepository},
};
use phoenix_db::DbPool;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateAccountInput {
    pub name: String,
    pub account_type: AccountType,
    pub institution: Option<String>,
    pub balance_cents: i64,
    pub currency: Currency,
}

#[derive(Debug)]
pub struct CreateTransactionInput {
    pub account_id: Uuid,
    pub amount_cents: i64,
    pub currency: Currency,
    pub transaction_type: TransactionType,
    pub category_id: Option<Uuid>,
    pub merchant: Option<String>,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub is_recurring: bool,
}

#[derive(Debug)]
pub struct CreateCategoryInput {
    pub name: String,
    pub category_type: CategoryType,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct TransactionFilter {
    pub account_id: Option<Uuid>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub category_id: Option<Uuid>,
    pub limit: i64,
    pub offset: i64,
}

pub struct CashFlow {
    pub total_income_cents: i64,
    pub total_expense_cents: i64,
    pub net_cents: i64,
    pub income_count: usize,
    pub expense_count: usize,
}

pub struct SpendingByCategory {
    pub category_id: Uuid,
    pub category_name: String,
    pub total_cents: i64,
    pub count: usize,
}

pub struct FinanceService {
    account_repo: SqlAccountRepository,
    tx_repo: SqlTransactionRepository,
    category_repo: SqlCategoryRepository,
    event_bus: Option<EventBus>,
}

impl FinanceService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            account_repo: SqlAccountRepository::new(pool.clone()),
            tx_repo: SqlTransactionRepository::new(pool.clone()),
            category_repo: SqlCategoryRepository::new(pool),
            event_bus: None,
        }
    }

    pub fn with_event_bus(pool: DbPool, event_bus: EventBus) -> Self {
        Self {
            account_repo: SqlAccountRepository::new(pool.clone()),
            tx_repo: SqlTransactionRepository::new(pool.clone()),
            category_repo: SqlCategoryRepository::new(pool),
            event_bus: Some(event_bus),
        }
    }

    // ─── Account Operations ───────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn create_account(&self, input: CreateAccountInput) -> anyhow::Result<Account> {
        let now = Utc::now();
        let account = Account {
            id: Uuid::new_v4(),
            name: input.name,
            account_type: input.account_type,
            institution: input.institution,
            balance_cents: input.balance_cents,
            currency: input.currency,
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        let created = self.account_repo.create(account).await?;
        if let Some(ref bus) = self.event_bus {
            bus.publish(DomainEvent::AccountCreated {
                account_id: created.id,
                name: created.name.clone(),
                timestamp: Utc::now(),
            })
            .await;
        }
        Ok(created)
    }

    #[instrument(skip(self))]
    pub async fn get_account(&self, id: Uuid) -> anyhow::Result<Option<Account>> {
        self.account_repo.get_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn list_accounts(&self) -> anyhow::Result<Vec<Account>> {
        self.account_repo.list_active().await
    }

    #[instrument(skip(self))]
    pub async fn update_account(&self, account: Account) -> anyhow::Result<Account> {
        self.account_repo.update(account).await
    }

    #[instrument(skip(self))]
    pub async fn deactivate_account(&self, id: Uuid) -> anyhow::Result<()> {
        self.account_repo.deactivate(id).await
    }

    // ─── Transaction Operations ───────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn create_transaction(
        &self,
        input: CreateTransactionInput,
    ) -> anyhow::Result<Transaction> {
        let now = Utc::now();
        let tx = Transaction {
            id: Uuid::new_v4(),
            account_id: input.account_id,
            amount_cents: input.amount_cents,
            currency: input.currency,
            transaction_type: input.transaction_type,
            category_id: input.category_id,
            merchant: input.merchant,
            description: input.description,
            date: input.date,
            tags: input.tags,
            is_recurring: input.is_recurring,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        let created = self.tx_repo.create(tx).await?;

        // Update account balance
        if let Some(mut account) = self.account_repo.get_by_id(created.account_id).await? {
            let delta = match created.transaction_type {
                TransactionType::Income => created.amount_cents,
                TransactionType::Expense => -created.amount_cents,
                TransactionType::Transfer => 0,
            };
            account.balance_cents += delta;
            self.account_repo.update(account).await?;
        }

        if let Some(ref bus) = self.event_bus {
            bus.publish(DomainEvent::TransactionCreated {
                transaction_id: created.id,
                account_id: created.account_id,
                amount_cents: created.amount_cents,
                timestamp: Utc::now(),
            })
            .await;
        }

        Ok(created)
    }

    #[instrument(skip(self))]
    pub async fn get_transaction(&self, id: Uuid) -> anyhow::Result<Option<Transaction>> {
        self.tx_repo.get_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn list_transactions(&self, account_id: Uuid) -> anyhow::Result<Vec<Transaction>> {
        self.tx_repo.list_by_account(account_id).await
    }

    #[instrument(skip(self))]
    pub async fn list_transactions_paginated(
        &self,
        filter: TransactionFilter,
    ) -> anyhow::Result<Vec<Transaction>> {
        match (filter.start_date, filter.end_date, filter.category_id, filter.account_id) {
            (Some(start), Some(end), _, _) => {
                let mut txs = self.tx_repo.list_by_date_range(start, end).await?;
                if let Some(account_id) = filter.account_id {
                    txs.retain(|t| t.account_id == account_id);
                }
                if let Some(cat_id) = filter.category_id {
                    txs.retain(|t| t.category_id == Some(cat_id));
                }
                txs.sort_by(|a, b| b.date.cmp(&a.date));
                let offset = filter.offset as usize;
                let limit = filter.limit as usize;
                Ok(txs.into_iter().skip(offset).take(limit).collect())
            }
            (_, _, Some(cat_id), _) => {
                let mut txs = self.tx_repo.list_by_category(cat_id).await?;
                if let Some(account_id) = filter.account_id {
                    txs.retain(|t| t.account_id == account_id);
                }
                let offset = filter.offset as usize;
                let limit = filter.limit as usize;
                Ok(txs.into_iter().skip(offset).take(limit).collect())
            }
            _ => self
                .tx_repo
                .list_paginated(filter.limit, filter.offset)
                .await,
        }
    }

    #[instrument(skip(self))]
    pub async fn search_transactions(&self, query: &str) -> anyhow::Result<Vec<Transaction>> {
        self.tx_repo.search(query).await
    }

    #[instrument(skip(self))]
    pub async fn soft_delete_transaction(&self, id: Uuid) -> anyhow::Result<()> {
        let tx = self
            .tx_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("transaction not found"))?;

        self.tx_repo.soft_delete(id).await?;

        // Reverse the balance change
        if let Some(mut account) = self.account_repo.get_by_id(tx.account_id).await? {
            let delta = match tx.transaction_type {
                TransactionType::Income => -tx.amount_cents,
                TransactionType::Expense => tx.amount_cents,
                TransactionType::Transfer => 0,
            };
            account.balance_cents += delta;
            self.account_repo.update(account).await?;
        }

        if let Some(ref bus) = self.event_bus {
            bus.publish(DomainEvent::TransactionDeleted {
                transaction_id: tx.id,
                account_id: tx.account_id,
                amount_cents: tx.amount_cents,
                timestamp: Utc::now(),
            })
            .await;
        }

        Ok(())
    }

    // ─── Category Operations ──────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn create_category(&self, input: CreateCategoryInput) -> anyhow::Result<Category> {
        let category = Category {
            id: Uuid::new_v4(),
            name: input.name,
            category_type: input.category_type,
            parent_id: input.parent_id,
            created_at: Utc::now(),
        };
        self.category_repo.create(category).await
    }

    #[instrument(skip(self))]
    pub async fn list_categories(&self) -> anyhow::Result<Vec<Category>> {
        self.category_repo.list_tree().await
    }

    #[instrument(skip(self))]
    pub async fn list_categories_by_type(
        &self,
        category_type: CategoryType,
    ) -> anyhow::Result<Vec<Category>> {
        self.category_repo.list_by_type(category_type).await
    }

    // ─── Balance / Reporting ──────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn get_balance(&self, account_id: Uuid) -> anyhow::Result<i64> {
        let account = self
            .account_repo
            .get_by_id(account_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("account not found"))?;
        Ok(account.balance_cents)
    }

    #[instrument(skip(self))]
    pub async fn get_net_worth(&self) -> anyhow::Result<i64> {
        let accounts = self.account_repo.list_active().await?;
        Ok(accounts.iter().map(|a| a.balance_cents).sum())
    }

    #[instrument(skip(self))]
    pub async fn get_cash_flow(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> anyhow::Result<CashFlow> {
        let txs = self.tx_repo.list_by_date_range(start_date, end_date).await?;
        let total_income_cents: i64 = txs
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Income)
            .map(|t| t.amount_cents)
            .sum();
        let total_expense_cents: i64 = txs
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Expense)
            .map(|t| t.amount_cents)
            .sum();
        let income_count = txs.iter().filter(|t| t.transaction_type == TransactionType::Income).count();
        let expense_count = txs.iter().filter(|t| t.transaction_type == TransactionType::Expense).count();

        Ok(CashFlow {
            total_income_cents,
            total_expense_cents,
            net_cents: total_income_cents - total_expense_cents,
            income_count,
            expense_count,
        })
    }

    #[instrument(skip(self))]
    pub async fn get_spending_by_category(
        &self,
        account_id: Option<Uuid>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> anyhow::Result<Vec<SpendingByCategory>> {
        let txs = self.tx_repo.list_by_date_range(start_date, end_date).await?;
        let expense_txs: Vec<&Transaction> = txs
            .iter()
            .filter(|t| {
                t.transaction_type == TransactionType::Expense
                    && account_id.map(|aid| t.account_id == aid).unwrap_or(true)
            })
            .collect();

        let mut by_category: Vec<(Uuid, i64, usize)> = Vec::new();
        for tx in &expense_txs {
            if let Some(cat_id) = tx.category_id {
                if let Some((_, total, count)) = by_category.iter_mut().find(|(id, _, _)| *id == cat_id) {
                    *total += tx.amount_cents;
                    *count += 1;
                } else {
                    by_category.push((cat_id, tx.amount_cents, 1));
                }
            }
        }

        let mut result = Vec::new();
        for (cat_id, total_cents, count) in by_category {
            let category_name = self
                .category_repo
                .get_by_id(cat_id)
                .await?
                .map(|c| c.name)
                .unwrap_or_else(|| "Unknown".into());
            result.push(SpendingByCategory {
                category_id: cat_id,
                category_name,
                total_cents,
                count,
            });
        }
        result.sort_by(|a, b| b.total_cents.cmp(&a.total_cents));
        Ok(result)
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
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS categories (
                id TEXT PRIMARY KEY, name TEXT NOT NULL,
                type TEXT NOT NULL, parent_id TEXT,
                created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_account() {
        let pool = create_test_db().await;
        let svc = FinanceService::new(pool);
        let account = svc
            .create_account(CreateAccountInput {
                name: "Checking".into(),
                account_type: AccountType::Checking,
                institution: Some("Bank".into()),
                balance_cents: 10000,
                currency: Currency::usd(),
            })
            .await
            .unwrap();
        assert_eq!(account.name, "Checking");
        assert!(account.is_active);
        let fetched = svc.get_account(account.id).await.unwrap();
        assert!(fetched.is_some());
    }

    #[tokio::test]
    async fn test_create_transaction_updates_balance() {
        let pool = create_test_db().await;
        let svc = FinanceService::new(pool);
        let account = svc
            .create_account(CreateAccountInput {
                name: "Checking".into(),
                account_type: AccountType::Checking,
                institution: None,
                balance_cents: 100000,
                currency: Currency::usd(),
            })
            .await
            .unwrap();

        let tx = svc
            .create_transaction(CreateTransactionInput {
                account_id: account.id,
                amount_cents: 5000,
                currency: Currency::usd(),
                transaction_type: TransactionType::Expense,
                category_id: None,
                merchant: Some("Store".into()),
                description: Some("Groceries".into()),
                date: Utc::now(),
                tags: vec!["food".into()],
                is_recurring: false,
            })
            .await
            .unwrap();
        assert_eq!(tx.amount_cents, 5000);

        let balance = svc.get_balance(account.id).await.unwrap();
        assert_eq!(balance, 95000);
    }

    #[tokio::test]
    async fn test_soft_delete_reverses_balance() {
        let pool = create_test_db().await;
        let svc = FinanceService::new(pool);
        let account = svc
            .create_account(CreateAccountInput {
                name: "Checking".into(),
                account_type: AccountType::Checking,
                institution: None,
                balance_cents: 100000,
                currency: Currency::usd(),
            })
            .await
            .unwrap();

        let tx = svc
            .create_transaction(CreateTransactionInput {
                account_id: account.id,
                amount_cents: 5000,
                currency: Currency::usd(),
                transaction_type: TransactionType::Expense,
                category_id: None,
                merchant: Some("Store".into()),
                description: Some("Groceries".into()),
                date: Utc::now(),
                tags: vec![],
                is_recurring: false,
            })
            .await
            .unwrap();

        assert_eq!(svc.get_balance(account.id).await.unwrap(), 95000);
        svc.soft_delete_transaction(tx.id).await.unwrap();
        assert_eq!(svc.get_balance(account.id).await.unwrap(), 100000);
    }

    #[tokio::test]
    async fn test_net_worth() {
        let pool = create_test_db().await;
        let svc = FinanceService::new(pool);
        svc.create_account(CreateAccountInput {
            name: "A".into(),
            account_type: AccountType::Checking,
            institution: None,
            balance_cents: 10000,
            currency: Currency::usd(),
        })
        .await
        .unwrap();
        svc.create_account(CreateAccountInput {
            name: "B".into(),
            account_type: AccountType::Savings,
            institution: None,
            balance_cents: 20000,
            currency: Currency::usd(),
        })
        .await
        .unwrap();
        assert_eq!(svc.get_net_worth().await.unwrap(), 30000);
    }

    #[tokio::test]
    async fn test_cash_flow() {
        let pool = create_test_db().await;
        let svc = FinanceService::new(pool);
        let account = svc
            .create_account(CreateAccountInput {
                name: "Checking".into(),
                account_type: AccountType::Checking,
                institution: None,
                balance_cents: 0,
                currency: Currency::usd(),
            })
            .await
            .unwrap();

        svc.create_transaction(CreateTransactionInput {
            account_id: account.id,
            amount_cents: 50000,
            currency: Currency::usd(),
            transaction_type: TransactionType::Income,
            category_id: None,
            merchant: None,
            description: Some("Salary".into()),
            date: Utc::now(),
            tags: vec![],
            is_recurring: true,
        })
        .await
        .unwrap();

        svc.create_transaction(CreateTransactionInput {
            account_id: account.id,
            amount_cents: 10000,
            currency: Currency::usd(),
            transaction_type: TransactionType::Expense,
            category_id: None,
            merchant: None,
            description: Some("Rent".into()),
            date: Utc::now(),
            tags: vec![],
            is_recurring: true,
        })
        .await
        .unwrap();

        let start = Utc::now() - chrono::Duration::hours(1);
        let end = Utc::now() + chrono::Duration::hours(1);
        let flow = svc.get_cash_flow(start, end).await.unwrap();
        assert_eq!(flow.total_income_cents, 50000);
        assert_eq!(flow.total_expense_cents, 10000);
        assert_eq!(flow.net_cents, 40000);
    }

    #[tokio::test]
    async fn test_categories() {
        let pool = create_test_db().await;
        let svc = FinanceService::new(pool);
        let cat = svc
            .create_category(CreateCategoryInput {
                name: "Groceries".into(),
                category_type: CategoryType::Expense,
                parent_id: None,
            })
            .await
            .unwrap();
        assert_eq!(cat.name, "Groceries");
        let cats = svc.list_categories_by_type(CategoryType::Expense).await.unwrap();
        assert_eq!(cats.len(), 1);
    }
}
