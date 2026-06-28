use async_trait::async_trait;
use chrono::{DateTime, Utc};
use phoenix_core::entities::{Budget, BudgetItem, BudgetPeriod};
use phoenix_core::value_objects::Currency;
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

// ─── Budget ─────────────────────────────────────────────────────────────────

#[async_trait]
pub trait BudgetRepository: Send + Sync {
    async fn create(&self, budget: Budget) -> anyhow::Result<Budget>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Budget>>;
    async fn update(&self, budget: Budget) -> anyhow::Result<Budget>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<Budget>>;
    async fn get_with_items(&self, id: Uuid) -> anyhow::Result<Option<(Budget, Vec<BudgetItem>)>>;
}

fn map_budget(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<Budget> {
    let period_str: String = row.get("period_type");
    let period = match period_str.as_str() {
        "weekly" => BudgetPeriod::Weekly,
        "monthly" => BudgetPeriod::Monthly,
        "yearly" => BudgetPeriod::Yearly,
        other => anyhow::bail!("invalid period_type: {other}"),
    };
    Ok(Budget {
        id: row.get::<String, _>("id").parse()?,
        name: row.get("name"),
        period,
        total_limit_cents: row.get("total_limit_cents"),
        currency: Currency::new(row.get::<String, _>("currency"))?,
        start_date: row.get::<String, _>("start_date").parse()?,
        end_date: row.get::<String, _>("end_date").parse()?,
        created_at: row.get::<String, _>("created_at").parse()?,
    })
}

fn period_str(p: &BudgetPeriod) -> &'static str {
    match p {
        BudgetPeriod::Weekly => "weekly",
        BudgetPeriod::Monthly => "monthly",
        BudgetPeriod::Yearly => "yearly",
    }
}

#[derive(Debug, Clone)]
pub struct SqlBudgetRepository {
    pool: DbPool,
}

impl SqlBudgetRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BudgetRepository for SqlBudgetRepository {
    async fn create(&self, budget: Budget) -> anyhow::Result<Budget> {
        sqlx::query(
            "INSERT INTO budgets (id, name, period_type, total_limit_cents, currency, start_date, end_date, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(budget.id.to_string())
        .bind(&budget.name)
        .bind(period_str(&budget.period))
        .bind(budget.total_limit_cents)
        .bind(budget.currency.code())
        .bind(budget.start_date.to_rfc3339())
        .bind(budget.end_date.to_rfc3339())
        .bind(budget.created_at.to_rfc3339())
        .bind(budget.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(budget.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created budget not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Budget>> {
        let row = sqlx::query("SELECT * FROM budgets WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_budget).transpose()
    }

    async fn update(&self, budget: Budget) -> anyhow::Result<Budget> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE budgets SET name = ?, period_type = ?, total_limit_cents = ?, currency = ?, start_date = ?, end_date = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&budget.name)
        .bind(period_str(&budget.period))
        .bind(budget.total_limit_cents)
        .bind(budget.currency.code())
        .bind(budget.start_date.to_rfc3339())
        .bind(budget.end_date.to_rfc3339())
        .bind(&now)
        .bind(budget.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(budget.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated budget not found"))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM budgets WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<Budget>> {
        let rows = sqlx::query(
            "SELECT * FROM budgets WHERE start_date <= ? AND end_date >= ? ORDER BY start_date",
        )
        .bind(end.to_rfc3339())
        .bind(start.to_rfc3339())
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_budget).collect()
    }

    async fn get_with_items(&self, id: Uuid) -> anyhow::Result<Option<(Budget, Vec<BudgetItem>)>> {
        let budget = self.get_by_id(id).await?;
        match budget {
            None => Ok(None),
            Some(b) => {
                let items = SqlBudgetItemRepository::new(self.pool.clone())
                    .list_by_budget(b.id)
                    .await?;
                Ok(Some((b, items)))
            }
        }
    }
}

// ─── BudgetItem ─────────────────────────────────────────────────────────────

#[async_trait]
pub trait BudgetItemRepository: Send + Sync {
    async fn create(&self, item: BudgetItem) -> anyhow::Result<BudgetItem>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<BudgetItem>>;
    async fn update(&self, item: BudgetItem) -> anyhow::Result<BudgetItem>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_by_budget(&self, budget_id: Uuid) -> anyhow::Result<Vec<BudgetItem>>;
}

fn map_budget_item(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<BudgetItem> {
    Ok(BudgetItem {
        id: row.get::<String, _>("id").parse()?,
        budget_id: row.get::<String, _>("budget_id").parse()?,
        category_id: row
            .get::<Option<String>, _>("category_id")
            .map(|s| s.parse())
            .transpose()?,
        limit_cents: row.get("limit_cents"),
        spent_cents: row.get("spent_cents"),
    })
}

#[derive(Debug, Clone)]
pub struct SqlBudgetItemRepository {
    pool: DbPool,
}

impl SqlBudgetItemRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BudgetItemRepository for SqlBudgetItemRepository {
    async fn create(&self, item: BudgetItem) -> anyhow::Result<BudgetItem> {
        sqlx::query(
            "INSERT INTO budget_items (id, budget_id, category_id, limit_cents, spent_cents) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(item.id.to_string())
        .bind(item.budget_id.to_string())
        .bind(item.category_id.map(|c| c.to_string()))
        .bind(item.limit_cents)
        .bind(item.spent_cents)
        .execute(&self.pool)
        .await?;
        self.get_by_id(item.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created budget item not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<BudgetItem>> {
        let row = sqlx::query("SELECT * FROM budget_items WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_budget_item).transpose()
    }

    async fn update(&self, item: BudgetItem) -> anyhow::Result<BudgetItem> {
        sqlx::query(
            "UPDATE budget_items SET budget_id = ?, category_id = ?, limit_cents = ?, spent_cents = ? WHERE id = ?",
        )
        .bind(item.budget_id.to_string())
        .bind(item.category_id.map(|c| c.to_string()))
        .bind(item.limit_cents)
        .bind(item.spent_cents)
        .bind(item.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(item.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated budget item not found"))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM budget_items WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_budget(&self, budget_id: Uuid) -> anyhow::Result<Vec<BudgetItem>> {
        let rows = sqlx::query("SELECT * FROM budget_items WHERE budget_id = ?")
            .bind(budget_id.to_string())
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_budget_item).collect()
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
        pool
    }

    #[tokio::test]
    async fn test_budget_crud() {
        let pool = create_test_db().await;
        let repo = SqlBudgetRepository::new(pool);
        let budget = Budget {
            id: Uuid::new_v4(),
            name: "Monthly Budget".into(),
            period: BudgetPeriod::Monthly,
            total_limit_cents: 200000,
            currency: Currency::usd(),
            start_date: Utc::now(),
            end_date: Utc::now(),
            created_at: Utc::now(),
        };
        let created = repo.create(budget).await.unwrap();
        assert_eq!(created.name, "Monthly Budget");
        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_some());
    }

    #[tokio::test]
    async fn test_budget_item() {
        let pool = create_test_db().await;
        let item_repo = SqlBudgetItemRepository::new(pool);
        let item = BudgetItem {
            id: Uuid::new_v4(),
            budget_id: Uuid::new_v4(),
            category_id: None,
            limit_cents: 50000,
            spent_cents: 0,
        };
        let created = item_repo.create(item).await.unwrap();
        assert_eq!(created.limit_cents, 50000);
    }
}
