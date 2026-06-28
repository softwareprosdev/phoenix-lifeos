use chrono::{DateTime, Utc};
use phoenix_core::entities::{Budget, BudgetItem, BudgetPeriod};
use phoenix_core::event_bus::{DomainEvent, EventBus};
use phoenix_core::value_objects::Currency;
use phoenix_db::repositories::{
    budget_repo::{
        BudgetItemRepository as _, BudgetRepository as _, SqlBudgetItemRepository,
        SqlBudgetRepository,
    },
    transaction_repo::{SqlTransactionRepository, TransactionRepository as _},
};
use phoenix_db::DbPool;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateBudgetInput {
    pub name: String,
    pub period: BudgetPeriod,
    pub total_limit_cents: i64,
    pub currency: Currency,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub items: Vec<CreateBudgetItemInput>,
}

#[derive(Debug)]
pub struct CreateBudgetItemInput {
    pub category_id: Option<Uuid>,
    pub limit_cents: i64,
}

#[derive(Debug)]
pub struct UpdateBudgetInput {
    pub id: Uuid,
    pub name: String,
    pub period: BudgetPeriod,
    pub total_limit_cents: i64,
    pub currency: Currency,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

pub struct BudgetProgress {
    pub budget: Budget,
    pub items: Vec<BudgetItemProgress>,
    pub total_spent_cents: i64,
    pub total_remaining_cents: i64,
    pub overall_pct: f64,
}

pub struct BudgetItemProgress {
    pub item: BudgetItem,
    pub spent_cents: i64,
    pub remaining_cents: i64,
    pub pct_used: f64,
    pub is_exceeded: bool,
}

pub struct ExceededCategory {
    pub category_id: Uuid,
    pub spent_cents: i64,
    pub limit_cents: i64,
}

pub struct BudgetService {
    budget_repo: SqlBudgetRepository,
    item_repo: SqlBudgetItemRepository,
    tx_repo: SqlTransactionRepository,
    event_bus: Option<EventBus>,
}

impl BudgetService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            budget_repo: SqlBudgetRepository::new(pool.clone()),
            item_repo: SqlBudgetItemRepository::new(pool.clone()),
            tx_repo: SqlTransactionRepository::new(pool),
            event_bus: None,
        }
    }

    pub fn with_event_bus(pool: DbPool, event_bus: EventBus) -> Self {
        Self {
            budget_repo: SqlBudgetRepository::new(pool.clone()),
            item_repo: SqlBudgetItemRepository::new(pool.clone()),
            tx_repo: SqlTransactionRepository::new(pool),
            event_bus: Some(event_bus),
        }
    }

    #[instrument(skip(self))]
    pub async fn create_budget(&self, input: CreateBudgetInput) -> anyhow::Result<(Budget, Vec<BudgetItem>)> {
        let now = Utc::now();
        let budget = Budget {
            id: Uuid::new_v4(),
            name: input.name,
            period: input.period,
            total_limit_cents: input.total_limit_cents,
            currency: input.currency,
            start_date: input.start_date,
            end_date: input.end_date,
            created_at: now,
        };
        let created = self.budget_repo.create(budget).await?;

        let mut items = Vec::new();
        for item_input in input.items {
            let item = BudgetItem {
                id: Uuid::new_v4(),
                budget_id: created.id,
                category_id: item_input.category_id,
                limit_cents: item_input.limit_cents,
                spent_cents: 0,
            };
            items.push(self.item_repo.create(item).await?);
        }

        Ok((created, items))
    }

    #[instrument(skip(self))]
    pub async fn get_budget(&self, id: Uuid) -> anyhow::Result<Option<Budget>> {
        self.budget_repo.get_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn list_budgets(&self) -> anyhow::Result<Vec<Budget>> {
        self.budget_repo
            .list_by_date_range(
                DateTime::UNIX_EPOCH,
                Utc::now() + chrono::Duration::days(365 * 10),
            )
            .await
    }

    #[instrument(skip(self))]
    pub async fn update_budget(&self, input: UpdateBudgetInput) -> anyhow::Result<Budget> {
        let budget = Budget {
            id: input.id,
            name: input.name,
            period: input.period,
            total_limit_cents: input.total_limit_cents,
            currency: input.currency,
            start_date: input.start_date,
            end_date: input.end_date,
            created_at: Utc::now(),
        };
        self.budget_repo.update(budget).await
    }

    #[instrument(skip(self))]
    pub async fn delete_budget(&self, id: Uuid) -> anyhow::Result<()> {
        self.budget_repo.delete(id).await
    }

    #[instrument(skip(self))]
    pub async fn get_budget_progress(&self, id: Uuid) -> anyhow::Result<Option<BudgetProgress>> {
        let result = self.budget_repo.get_with_items(id).await?;
        match result {
            None => Ok(None),
            Some((budget, items)) => {
                let mut item_progresses = Vec::new();
                let mut total_spent_cents = 0i64;

                for item in &items {
                    let spent = if let Some(cat_id) = item.category_id {
                        let txs = self.tx_repo.list_by_category(cat_id).await?;
                        let filtered: i64 = txs
                            .iter()
                            .filter(|t| {
                                t.date >= budget.start_date && t.date <= budget.end_date
                            })
                            .map(|t| t.amount_cents)
                            .sum();
                        filtered
                    } else {
                        0
                    };

                    total_spent_cents += spent;
                    item_progresses.push(BudgetItemProgress {
                        spent_cents: spent,
                        remaining_cents: item.limit_cents - spent,
                        pct_used: if item.limit_cents > 0 {
                            (spent as f64 / item.limit_cents as f64) * 100.0
                        } else {
                            0.0
                        },
                        is_exceeded: spent > item.limit_cents,
                        item: item.clone(),
                    });
                }

                Ok(Some(BudgetProgress {
                    total_spent_cents,
                    total_remaining_cents: budget.total_limit_cents - total_spent_cents,
                    overall_pct: if budget.total_limit_cents > 0 {
                        (total_spent_cents as f64 / budget.total_limit_cents as f64) * 100.0
                    } else {
                        0.0
                    },
                    budget,
                    items: item_progresses,
                }))
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn check_budget_exceeded(&self, budget_id: Uuid) -> anyhow::Result<Vec<ExceededCategory>> {
        let progress = self
            .get_budget_progress(budget_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("budget not found"))?;

        let mut exceeded = Vec::new();
        for ip in &progress.items {
            if ip.is_exceeded {
                if let Some(cat_id) = ip.item.category_id {
                    exceeded.push(ExceededCategory {
                        category_id: cat_id,
                        spent_cents: ip.spent_cents,
                        limit_cents: ip.item.limit_cents,
                    });

                    if let Some(ref bus) = self.event_bus {
                        bus.publish(DomainEvent::BudgetExceeded {
                            budget_id,
                            category_id: cat_id,
                            spent_cents: ip.spent_cents,
                            limit_cents: ip.item.limit_cents,
                            timestamp: Utc::now(),
                        })
                        .await;
                    }
                }
            }
        }

        Ok(exceeded)
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
            "CREATE TABLE IF NOT EXISTS budgets (
                id TEXT PRIMARY KEY, name TEXT NOT NULL,
                period_type TEXT NOT NULL, total_limit_cents INTEGER NOT NULL,
                currency TEXT NOT NULL DEFAULT 'USD',
                start_date TEXT NOT NULL, end_date TEXT NOT NULL,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS budget_items (
                id TEXT PRIMARY KEY, budget_id TEXT NOT NULL,
                category_id TEXT, limit_cents INTEGER NOT NULL, spent_cents INTEGER NOT NULL DEFAULT 0
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
        pool
    }

    #[tokio::test]
    async fn test_create_budget() {
        let pool = create_test_db().await;
        let svc = BudgetService::new(pool);
        let (budget, items) = svc
            .create_budget(CreateBudgetInput {
                name: "Monthly".into(),
                period: BudgetPeriod::Monthly,
                total_limit_cents: 200000,
                currency: Currency::usd(),
                start_date: Utc::now(),
                end_date: Utc::now() + chrono::Duration::days(30),
                items: vec![CreateBudgetItemInput {
                    category_id: None,
                    limit_cents: 50000,
                }],
            })
            .await
            .unwrap();
        assert_eq!(budget.name, "Monthly");
        assert_eq!(items.len(), 1);
    }

    #[tokio::test]
    async fn test_budget_progress() {
        let pool = create_test_db().await;
        let svc = BudgetService::new(pool);
        let (budget, _) = svc
            .create_budget(CreateBudgetInput {
                name: "Monthly".into(),
                period: BudgetPeriod::Monthly,
                total_limit_cents: 200000,
                currency: Currency::usd(),
                start_date: Utc::now() - chrono::Duration::days(1),
                end_date: Utc::now() + chrono::Duration::days(29),
                items: vec![],
            })
            .await
            .unwrap();

        let progress = svc.get_budget_progress(budget.id).await.unwrap();
        assert!(progress.is_some());
        let p = progress.unwrap();
        assert_eq!(p.total_spent_cents, 0);
    }
}
