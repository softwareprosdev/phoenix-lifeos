use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DomainEvent {
    TransactionCreated {
        transaction_id: Uuid,
        account_id: Uuid,
        amount_cents: i64,
        timestamp: DateTime<Utc>,
    },
    TransactionDeleted {
        transaction_id: Uuid,
        account_id: Uuid,
        amount_cents: i64,
        timestamp: DateTime<Utc>,
    },
    AccountCreated {
        account_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },
    BudgetExceeded {
        budget_id: Uuid,
        category_id: Uuid,
        spent_cents: i64,
        limit_cents: i64,
        timestamp: DateTime<Utc>,
    },
    DebtPaymentMade {
        debt_id: Uuid,
        amount_cents: i64,
        remaining_cents: i64,
        timestamp: DateTime<Utc>,
    },
    CreditScoreUpdated {
        report_id: Uuid,
        score: i32,
        timestamp: DateTime<Utc>,
    },
    GoalProgressUpdated {
        goal_id: Uuid,
        progress_pct: f64,
        timestamp: DateTime<Utc>,
    },
    GoalAchieved {
        goal_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },
    VaultItemAccessed {
        item_id: Uuid,
        action: String,
        timestamp: DateTime<Utc>,
    },
    DocumentAdded {
        document_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },
}

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    fn interested_in(&self) -> Vec<DomainEvent>;
    async fn handle(&self, event: &DomainEvent);
}

pub struct EventBus {
    handlers: tokio::sync::Mutex<Vec<Box<dyn EventHandler>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: tokio::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn subscribe(&mut self, handler: Box<dyn EventHandler>) {
        if let Ok(mut handlers) = self.handlers.try_lock() {
            handlers.push(handler);
        }
    }

    pub async fn publish(&self, event: DomainEvent) {
        let handlers = self.handlers.lock().await;
        for handler in handlers.iter() {
            if handler.interested_in().iter().any(|e| std::mem::discriminant(e) == std::mem::discriminant(&event))
            {
                handler.handle(&event).await;
            }
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
