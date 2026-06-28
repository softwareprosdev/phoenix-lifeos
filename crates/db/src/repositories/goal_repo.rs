use async_trait::async_trait;
use chrono::Utc;
use phoenix_core::entities::{Goal, GoalType};
use phoenix_core::value_objects::Currency;
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

#[async_trait]
pub trait GoalRepository: Send + Sync {
    async fn create(&self, goal: Goal) -> anyhow::Result<Goal>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Goal>>;
    async fn update(&self, goal: Goal) -> anyhow::Result<Goal>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_by_type(&self, goal_type: GoalType) -> anyhow::Result<Vec<Goal>>;
    async fn list_active(&self) -> anyhow::Result<Vec<Goal>>;
    async fn list_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<Goal>>;
}

#[derive(Debug, Clone)]
pub struct SqlGoalRepository {
    pool: DbPool,
}

impl SqlGoalRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn map_goal(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<Goal> {
    let gt_str: String = row.get("goal_type");
    let goal_type = match gt_str.as_str() {
        "savings" => GoalType::Savings,
        "emergency_fund" => GoalType::EmergencyFund,
        "home_purchase" => GoalType::HomePurchase,
        "vehicle" => GoalType::Vehicle,
        "retirement" => GoalType::Retirement,
        "education" => GoalType::Education,
        "vacation" => GoalType::Vacation,
        "debt_payoff" => GoalType::DebtPayoff,
        "custom" => GoalType::Custom,
        other => anyhow::bail!("invalid goal_type: {other}"),
    };
    Ok(Goal {
        id: row.get::<String, _>("id").parse()?,
        name: row.get("name"),
        goal_type,
        target_amount_cents: row.get("target_amount_cents"),
        current_amount_cents: row.get("current_amount_cents"),
        monthly_contribution_cents: row.get("monthly_contribution_cents"),
        currency: Currency::new(row.get::<String, _>("currency"))?,
        deadline: row
            .get::<Option<String>, _>("deadline")
            .map(|s| s.parse())
            .transpose()?,
        linked_account_id: row
            .get::<Option<String>, _>("linked_account_id")
            .map(|s| s.parse())
            .transpose()?,
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn goal_type_str(g: &GoalType) -> &'static str {
    match g {
        GoalType::Savings => "savings",
        GoalType::EmergencyFund => "emergency_fund",
        GoalType::HomePurchase => "home_purchase",
        GoalType::Vehicle => "vehicle",
        GoalType::Retirement => "retirement",
        GoalType::Education => "education",
        GoalType::Vacation => "vacation",
        GoalType::DebtPayoff => "debt_payoff",
        GoalType::Custom => "custom",
    }
}

#[async_trait]
impl GoalRepository for SqlGoalRepository {
    async fn create(&self, goal: Goal) -> anyhow::Result<Goal> {
        sqlx::query(
            "INSERT INTO goals (id, name, goal_type, target_amount_cents, current_amount_cents, monthly_contribution_cents, currency, deadline, linked_account_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(goal.id.to_string())
        .bind(&goal.name)
        .bind(goal_type_str(&goal.goal_type))
        .bind(goal.target_amount_cents)
        .bind(goal.current_amount_cents)
        .bind(goal.monthly_contribution_cents)
        .bind(goal.currency.code())
        .bind(goal.deadline.map(|d| d.to_rfc3339()))
        .bind(goal.linked_account_id.map(|a| a.to_string()))
        .bind(goal.created_at.to_rfc3339())
        .bind(goal.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(goal.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created goal not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Goal>> {
        let row = sqlx::query("SELECT * FROM goals WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_goal).transpose()
    }

    async fn update(&self, goal: Goal) -> anyhow::Result<Goal> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE goals SET name = ?, goal_type = ?, target_amount_cents = ?, current_amount_cents = ?, monthly_contribution_cents = ?, currency = ?, deadline = ?, linked_account_id = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&goal.name)
        .bind(goal_type_str(&goal.goal_type))
        .bind(goal.target_amount_cents)
        .bind(goal.current_amount_cents)
        .bind(goal.monthly_contribution_cents)
        .bind(goal.currency.code())
        .bind(goal.deadline.map(|d| d.to_rfc3339()))
        .bind(goal.linked_account_id.map(|a| a.to_string()))
        .bind(&now)
        .bind(goal.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(goal.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated goal not found"))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM goals WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_type(&self, goal_type: GoalType) -> anyhow::Result<Vec<Goal>> {
        let rows = sqlx::query("SELECT * FROM goals WHERE goal_type = ? ORDER BY name")
            .bind(goal_type_str(&goal_type))
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_goal).collect()
    }

    async fn list_active(&self) -> anyhow::Result<Vec<Goal>> {
        let rows = sqlx::query(
            "SELECT * FROM goals WHERE current_amount_cents < target_amount_cents ORDER BY name",
        )
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_goal).collect()
    }

    async fn list_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<Goal>> {
        let rows = sqlx::query("SELECT * FROM goals WHERE linked_account_id = ? ORDER BY name")
            .bind(account_id.to_string())
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_goal).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phoenix_core::value_objects::Currency;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS goals (
                id TEXT PRIMARY KEY, name TEXT NOT NULL,
                goal_type TEXT NOT NULL,
                target_amount_cents INTEGER NOT NULL,
                current_amount_cents INTEGER NOT NULL DEFAULT 0,
                monthly_contribution_cents INTEGER NOT NULL DEFAULT 0,
                currency TEXT NOT NULL DEFAULT 'USD',
                deadline TEXT, linked_account_id TEXT,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    fn test_goal() -> Goal {
        Goal {
            id: Uuid::new_v4(),
            name: "Emergency Fund".into(),
            goal_type: GoalType::EmergencyFund,
            target_amount_cents: 1000000,
            current_amount_cents: 250000,
            monthly_contribution_cents: 50000,
            currency: Currency::usd(),
            deadline: None,
            linked_account_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let pool = create_test_db().await;
        let repo = SqlGoalRepository::new(pool);
        let goal = test_goal();
        let created = repo.create(goal).await.unwrap();
        assert_eq!(created.name, "Emergency Fund");
        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_some());
    }

    #[tokio::test]
    async fn test_list_active() {
        let pool = create_test_db().await;
        let repo = SqlGoalRepository::new(pool);
        let goal = test_goal();
        repo.create(goal).await.unwrap();
        let mut achieved = test_goal();
        achieved.id = Uuid::new_v4();
        achieved.current_amount_cents = achieved.target_amount_cents;
        repo.create(achieved).await.unwrap();
        let active = repo.list_active().await.unwrap();
        assert_eq!(active.len(), 1);
    }
}
