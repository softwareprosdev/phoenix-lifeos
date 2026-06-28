use chrono::{DateTime, Utc};
use phoenix_core::entities::{Goal, GoalType};
use phoenix_core::event_bus::{DomainEvent, EventBus};
use phoenix_core::value_objects::Currency;
use phoenix_db::repositories::goal_repo::{GoalRepository as _, SqlGoalRepository};
use phoenix_db::DbPool;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateGoalInput {
    pub name: String,
    pub goal_type: GoalType,
    pub target_amount_cents: i64,
    pub current_amount_cents: i64,
    pub monthly_contribution_cents: i64,
    pub currency: Currency,
    pub deadline: Option<DateTime<Utc>>,
    pub linked_account_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct UpdateGoalInput {
    pub id: Uuid,
    pub name: String,
    pub goal_type: GoalType,
    pub target_amount_cents: i64,
    pub current_amount_cents: i64,
    pub monthly_contribution_cents: i64,
    pub currency: Currency,
    pub deadline: Option<DateTime<Utc>>,
    pub linked_account_id: Option<Uuid>,
}

pub struct GoalProgress {
    pub goal: Goal,
    pub progress_pct: f64,
    pub remaining_cents: i64,
    pub is_achieved: bool,
}

pub struct GoalService {
    goal_repo: SqlGoalRepository,
    event_bus: Option<EventBus>,
}

impl GoalService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            goal_repo: SqlGoalRepository::new(pool),
            event_bus: None,
        }
    }

    pub fn with_event_bus(pool: DbPool, event_bus: EventBus) -> Self {
        Self {
            goal_repo: SqlGoalRepository::new(pool),
            event_bus: Some(event_bus),
        }
    }

    #[instrument(skip(self))]
    pub async fn create_goal(&self, input: CreateGoalInput) -> anyhow::Result<Goal> {
        if input.target_amount_cents <= 0 {
            anyhow::bail!("goal target must be greater than zero");
        }
        let now = Utc::now();
        let goal = Goal {
            id: Uuid::new_v4(),
            name: input.name,
            goal_type: input.goal_type,
            target_amount_cents: input.target_amount_cents,
            current_amount_cents: input.current_amount_cents,
            monthly_contribution_cents: input.monthly_contribution_cents,
            currency: input.currency,
            deadline: input.deadline,
            linked_account_id: input.linked_account_id,
            created_at: now,
            updated_at: now,
        };
        self.goal_repo.create(goal).await
    }

    #[instrument(skip(self))]
    pub async fn get_goal(&self, id: Uuid) -> anyhow::Result<Option<Goal>> {
        self.goal_repo.get_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn list_goals(&self) -> anyhow::Result<Vec<Goal>> {
        self.goal_repo.list_active().await
    }

    #[instrument(skip(self))]
    pub async fn list_goals_by_type(&self, goal_type: GoalType) -> anyhow::Result<Vec<Goal>> {
        self.goal_repo.list_by_type(goal_type).await
    }

    #[instrument(skip(self))]
    pub async fn list_active_goals(&self) -> anyhow::Result<Vec<Goal>> {
        self.goal_repo.list_active().await
    }

    #[instrument(skip(self))]
    pub async fn update_goal(&self, input: UpdateGoalInput) -> anyhow::Result<Goal> {
        let now = Utc::now();
        let goal = Goal {
            id: input.id,
            name: input.name,
            goal_type: input.goal_type,
            target_amount_cents: input.target_amount_cents,
            current_amount_cents: input.current_amount_cents,
            monthly_contribution_cents: input.monthly_contribution_cents,
            currency: input.currency,
            deadline: input.deadline,
            linked_account_id: input.linked_account_id,
            created_at: now,
            updated_at: now,
        };
        let updated = self.goal_repo.update(goal).await?;

        self.emit_progress_events(&updated).await;

        Ok(updated)
    }

    #[instrument(skip(self))]
    pub async fn contribute_to_goal(&self, goal_id: Uuid, amount_cents: i64) -> anyhow::Result<Goal> {
        if amount_cents <= 0 {
            anyhow::bail!("contribution must be positive");
        }
        let mut goal = self
            .goal_repo
            .get_by_id(goal_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("goal not found"))?;

        goal.current_amount_cents = (goal.current_amount_cents + amount_cents).min(goal.target_amount_cents);
        goal.updated_at = Utc::now();
        let updated = self.goal_repo.update(goal).await?;

        self.emit_progress_events(&updated).await;

        Ok(updated)
    }

    #[instrument(skip(self))]
    pub async fn delete_goal(&self, id: Uuid) -> anyhow::Result<()> {
        self.goal_repo.delete(id).await
    }

    #[instrument(skip(self))]
    pub async fn get_goal_progress(&self, id: Uuid) -> anyhow::Result<Option<GoalProgress>> {
        let goal = self.goal_repo.get_by_id(id).await?;
        Ok(goal.map(|g| GoalProgress {
            progress_pct: g.progress_percentage(),
            remaining_cents: g.remaining_amount_cents(),
            is_achieved: g.is_achieved(),
            goal: g,
        }))
    }

    async fn emit_progress_events(&self, goal: &Goal) {
        if let Some(ref bus) = self.event_bus {
            let was_previously_achieved = goal.current_amount_cents >= goal.target_amount_cents;

            bus.publish(DomainEvent::GoalProgressUpdated {
                goal_id: goal.id,
                progress_pct: goal.progress_percentage(),
                timestamp: Utc::now(),
            })
            .await;

            if was_previously_achieved && goal.is_achieved() {
                bus.publish(DomainEvent::GoalAchieved {
                    goal_id: goal.id,
                    name: goal.name.clone(),
                    timestamp: Utc::now(),
                })
                .await;
            }
        }
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

    #[tokio::test]
    async fn test_create_goal() {
        let pool = create_test_db().await;
        let svc = GoalService::new(pool);
        let goal = svc
            .create_goal(CreateGoalInput {
                name: "Emergency Fund".into(),
                goal_type: GoalType::EmergencyFund,
                target_amount_cents: 1000000,
                current_amount_cents: 0,
                monthly_contribution_cents: 50000,
                currency: Currency::usd(),
                deadline: None,
                linked_account_id: None,
            })
            .await
            .unwrap();
        assert_eq!(goal.name, "Emergency Fund");
        assert_eq!(goal.target_amount_cents, 1000000);
    }

    #[tokio::test]
    async fn test_contribute_to_goal() {
        let pool = create_test_db().await;
        let svc = GoalService::new(pool);
        let goal = svc
            .create_goal(CreateGoalInput {
                name: "Emergency Fund".into(),
                goal_type: GoalType::EmergencyFund,
                target_amount_cents: 1000000,
                current_amount_cents: 0,
                monthly_contribution_cents: 50000,
                currency: Currency::usd(),
                deadline: None,
                linked_account_id: None,
            })
            .await
            .unwrap();

        let updated = svc.contribute_to_goal(goal.id, 250000).await.unwrap();
        assert_eq!(updated.current_amount_cents, 250000);
        assert_eq!(updated.progress_percentage(), 25.0);
    }

    #[tokio::test]
    async fn test_goal_achieved() {
        let pool = create_test_db().await;
        let svc = GoalService::new(pool);
        let goal = svc
            .create_goal(CreateGoalInput {
                name: "Small Goal".into(),
                goal_type: GoalType::Savings,
                target_amount_cents: 1000,
                current_amount_cents: 0,
                monthly_contribution_cents: 100,
                currency: Currency::usd(),
                deadline: None,
                linked_account_id: None,
            })
            .await
            .unwrap();

        let updated = svc.contribute_to_goal(goal.id, 1000).await.unwrap();
        assert!(updated.is_achieved());
        assert_eq!(updated.progress_percentage(), 100.0);
    }

    #[tokio::test]
    async fn test_goal_progress() {
        let pool = create_test_db().await;
        let svc = GoalService::new(pool);
        let goal = svc
            .create_goal(CreateGoalInput {
                name: "Vacation".into(),
                goal_type: GoalType::Vacation,
                target_amount_cents: 500000,
                current_amount_cents: 100000,
                monthly_contribution_cents: 20000,
                currency: Currency::usd(),
                deadline: None,
                linked_account_id: None,
            })
            .await
            .unwrap();

        let progress = svc.get_goal_progress(goal.id).await.unwrap().unwrap();
        assert_eq!(progress.progress_pct, 20.0);
        assert_eq!(progress.remaining_cents, 400000);
        assert!(!progress.is_achieved);
    }
}
