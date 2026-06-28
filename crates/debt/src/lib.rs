use chrono::Utc;
use phoenix_core::entities::{Debt, DebtType};
use phoenix_core::event_bus::{DomainEvent, EventBus};
use phoenix_core::value_objects::{Currency, Percentage};
use phoenix_db::repositories::debt_repo::{DebtRepository as _, SqlDebtRepository};
use phoenix_db::DbPool;
use rust_decimal::Decimal;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateDebtInput {
    pub name: String,
    pub debt_type: DebtType,
    pub principal_cents: i64,
    pub remaining_balance_cents: i64,
    pub interest_rate_pct: Decimal,
    pub minimum_payment_cents: i64,
    pub currency: Currency,
    pub due_date: Option<chrono::DateTime<Utc>>,
    pub creditor: Option<String>,
}

#[derive(Debug)]
pub struct UpdateDebtInput {
    pub id: Uuid,
    pub name: String,
    pub debt_type: DebtType,
    pub principal_cents: i64,
    pub remaining_balance_cents: i64,
    pub interest_rate_pct: Decimal,
    pub minimum_payment_cents: i64,
    pub currency: Currency,
    pub due_date: Option<chrono::DateTime<Utc>>,
    pub creditor: Option<String>,
}

#[derive(Debug)]
pub enum PayoffStrategy {
    Snowball,
    Avalanche,
}

pub struct PayoffMonth {
    pub month: u32,
    pub payment_cents: i64,
    pub principal_paid_cents: i64,
    pub interest_paid_cents: i64,
    pub remaining_cents: i64,
}

pub struct PayoffSchedule {
    pub debt: Debt,
    pub months: Vec<PayoffMonth>,
    pub total_interest_cents: i64,
    pub total_paid_cents: i64,
    pub months_to_payoff: u32,
}

pub struct DebtSummary {
    pub total_balance_cents: i64,
    pub total_principal_cents: i64,
    pub debt_count: usize,
    pub average_apr_pct: Decimal,
    pub total_minimum_payment_cents: i64,
}

pub struct DebtService {
    debt_repo: SqlDebtRepository,
    event_bus: Option<EventBus>,
}

impl DebtService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            debt_repo: SqlDebtRepository::new(pool),
            event_bus: None,
        }
    }

    pub fn with_event_bus(pool: DbPool, event_bus: EventBus) -> Self {
        Self {
            debt_repo: SqlDebtRepository::new(pool),
            event_bus: Some(event_bus),
        }
    }

    #[instrument(skip(self))]
    pub async fn create_debt(&self, input: CreateDebtInput) -> anyhow::Result<Debt> {
        let now = Utc::now();
        let debt = Debt {
            id: Uuid::new_v4(),
            name: input.name,
            debt_type: input.debt_type,
            principal_cents: input.principal_cents,
            remaining_balance_cents: input.remaining_balance_cents,
            interest_rate: Percentage::new(input.interest_rate_pct)?,
            minimum_payment_cents: input.minimum_payment_cents,
            currency: input.currency,
            due_date: input.due_date,
            creditor: input.creditor,
            created_at: now,
            updated_at: now,
        };
        self.debt_repo.create(debt).await
    }

    #[instrument(skip(self))]
    pub async fn get_debt(&self, id: Uuid) -> anyhow::Result<Option<Debt>> {
        self.debt_repo.get_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn list_debts(&self) -> anyhow::Result<Vec<Debt>> {
        self.debt_repo.list_active().await
    }

    #[instrument(skip(self))]
    pub async fn update_debt(&self, input: UpdateDebtInput) -> anyhow::Result<Debt> {
        let now = Utc::now();
        let debt = Debt {
            id: input.id,
            name: input.name,
            debt_type: input.debt_type,
            principal_cents: input.principal_cents,
            remaining_balance_cents: input.remaining_balance_cents,
            interest_rate: Percentage::new(input.interest_rate_pct)?,
            minimum_payment_cents: input.minimum_payment_cents,
            currency: input.currency,
            due_date: input.due_date,
            creditor: input.creditor,
            created_at: now,
            updated_at: now,
        };
        self.debt_repo.update(debt).await
    }

    #[instrument(skip(self))]
    pub async fn delete_debt(&self, id: Uuid) -> anyhow::Result<()> {
        self.debt_repo.delete(id).await
    }

    #[instrument(skip(self))]
    pub async fn make_payment(&self, debt_id: Uuid, amount_cents: i64) -> anyhow::Result<Debt> {
        if amount_cents <= 0 {
            anyhow::bail!("payment amount must be positive");
        }
        let mut debt = self
            .debt_repo
            .get_by_id(debt_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("debt not found"))?;

        if amount_cents > debt.remaining_balance_cents {
            debt.remaining_balance_cents = 0;
        } else {
            debt.remaining_balance_cents -= amount_cents;
        }
        debt.updated_at = Utc::now();

        let updated = self.debt_repo.update(debt).await?;

        if let Some(ref bus) = self.event_bus {
            bus.publish(DomainEvent::DebtPaymentMade {
                debt_id: updated.id,
                amount_cents,
                remaining_cents: updated.remaining_balance_cents,
                timestamp: Utc::now(),
            })
            .await;
        }

        Ok(updated)
    }

    #[instrument(skip(self))]
    pub async fn get_payoff_schedule(
        &self,
        debt_id: Uuid,
        strategy: PayoffStrategy,
    ) -> anyhow::Result<Option<PayoffSchedule>> {
        match strategy {
            PayoffStrategy::Snowball => self.calculate_snowball_schedule(vec![debt_id]).await,
            PayoffStrategy::Avalanche => self.calculate_avalanche_schedule(vec![debt_id]).await,
        }
        .map(|v| v.into_iter().next())
    }

    #[instrument(skip(self))]
    pub async fn calculate_snowball_schedule(
        &self,
        debt_ids: Vec<Uuid>,
    ) -> anyhow::Result<Vec<PayoffSchedule>> {
        let mut debts = Vec::new();
        for id in debt_ids {
            if let Some(d) = self.debt_repo.get_by_id(id).await? {
                debts.push(d);
            }
        }

        // Snowball: sort by smallest remaining balance first
        debts.sort_by_key(|d| d.remaining_balance_cents);
        Ok(calculate_schedules(&debts))
    }

    #[instrument(skip(self))]
    pub async fn calculate_avalanche_schedule(
        &self,
        debt_ids: Vec<Uuid>,
    ) -> anyhow::Result<Vec<PayoffSchedule>> {
        let mut debts = Vec::new();
        for id in debt_ids {
            if let Some(d) = self.debt_repo.get_by_id(id).await? {
                debts.push(d);
            }
        }

        // Avalanche: sort by highest APR first
        debts.sort_by(|a, b| {
            b.interest_rate
                .value()
                .cmp(&a.interest_rate.value())
        });
        Ok(calculate_schedules(&debts))
    }

    #[instrument(skip(self))]
    pub async fn get_total_debt(&self) -> anyhow::Result<i64> {
        let debts = self.debt_repo.list_active().await?;
        Ok(debts.iter().map(|d| d.remaining_balance_cents).sum())
    }

    #[instrument(skip(self))]
    pub async fn get_debt_summary(&self) -> anyhow::Result<DebtSummary> {
        let debts = self.debt_repo.list_active().await?;
        let count = debts.len();
        let total_balance: i64 = debts.iter().map(|d| d.remaining_balance_cents).sum();
        let total_principal: i64 = debts.iter().map(|d| d.principal_cents).sum();
        let total_min_payment: i64 = debts.iter().map(|d| d.minimum_payment_cents).sum();
        let avg_apr = if count > 0 {
            let sum: Decimal = debts.iter().map(|d| d.interest_rate.value()).sum();
            sum / Decimal::from(count)
        } else {
            Decimal::ZERO
        };

        Ok(DebtSummary {
            total_balance_cents: total_balance,
            total_principal_cents: total_principal,
            debt_count: count,
            average_apr_pct: avg_apr,
            total_minimum_payment_cents: total_min_payment,
        })
    }
}

fn calculate_schedules(debts: &[Debt]) -> Vec<PayoffSchedule> {
    debts
        .iter()
        .map(|debt| {
            let mut remaining = debt.remaining_balance_cents;
            let monthly_rate = debt.interest_rate.as_rate() / Decimal::from(12);
            let min_payment = debt.minimum_payment_cents.max(1);
            let mut months = Vec::new();
            let mut total_interest = 0i64;
            let mut total_paid = 0i64;
            let mut month_num = 0u32;

            while remaining > 0 && month_num < 600 {
                month_num += 1;
                let interest = (Decimal::from(remaining) * monthly_rate).round().try_into().unwrap_or(0i64);
                let mut principal_paid = min_payment - interest;
                if principal_paid <= 0 {
                    principal_paid = 1;
                }
                if principal_paid > remaining {
                    principal_paid = remaining;
                }
                let actual_payment = interest + principal_paid;

                remaining -= principal_paid;
                total_interest += interest;
                total_paid += actual_payment;

                months.push(PayoffMonth {
                    month: month_num,
                    payment_cents: actual_payment,
                    principal_paid_cents: principal_paid,
                    interest_paid_cents: interest,
                    remaining_cents: remaining.max(0),
                });
            }

            PayoffSchedule {
                debt: debt.clone(),
                months,
                total_interest_cents: total_interest,
                total_paid_cents: total_paid,
                months_to_payoff: month_num,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use phoenix_core::value_objects::Currency;
    use rust_decimal::Decimal;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS debts (
                id TEXT PRIMARY KEY, name TEXT NOT NULL,
                debt_type TEXT NOT NULL, principal_cents INTEGER NOT NULL,
                remaining_balance_cents INTEGER NOT NULL,
                interest_rate_pct TEXT NOT NULL,
                minimum_payment_cents INTEGER NOT NULL,
                currency TEXT NOT NULL DEFAULT 'USD',
                due_date TEXT, creditor TEXT,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_debt() {
        let pool = create_test_db().await;
        let svc = DebtService::new(pool);
        let debt = svc
            .create_debt(CreateDebtInput {
                name: "Credit Card".into(),
                debt_type: DebtType::CreditCard,
                principal_cents: 100000,
                remaining_balance_cents: 50000,
                interest_rate_pct: Decimal::new(1899, 2),
                minimum_payment_cents: 2500,
                currency: Currency::usd(),
                due_date: None,
                creditor: Some("Chase".into()),
            })
            .await
            .unwrap();
        assert_eq!(debt.name, "Credit Card");
        assert_eq!(debt.remaining_balance_cents, 50000);
    }

    #[tokio::test]
    async fn test_make_payment() {
        let pool = create_test_db().await;
        let svc = DebtService::new(pool);
        let debt = svc
            .create_debt(CreateDebtInput {
                name: "Loan".into(),
                debt_type: DebtType::PersonalLoan,
                principal_cents: 100000,
                remaining_balance_cents: 100000,
                interest_rate_pct: Decimal::new(500, 2),
                minimum_payment_cents: 10000,
                currency: Currency::usd(),
                due_date: None,
                creditor: None,
            })
            .await
            .unwrap();

        let updated = svc.make_payment(debt.id, 25000).await.unwrap();
        assert_eq!(updated.remaining_balance_cents, 75000);
    }

    #[tokio::test]
    async fn test_debt_summary() {
        let pool = create_test_db().await;
        let svc = DebtService::new(pool);
        svc.create_debt(CreateDebtInput {
            name: "Card A".into(),
            debt_type: DebtType::CreditCard,
            principal_cents: 50000,
            remaining_balance_cents: 25000,
            interest_rate_pct: Decimal::new(1899, 2),
            minimum_payment_cents: 1000,
            currency: Currency::usd(),
            due_date: None,
            creditor: None,
        })
        .await
        .unwrap();
        svc.create_debt(CreateDebtInput {
            name: "Card B".into(),
            debt_type: DebtType::CreditCard,
            principal_cents: 100000,
            remaining_balance_cents: 75000,
            interest_rate_pct: Decimal::new(1500, 2),
            minimum_payment_cents: 2000,
            currency: Currency::usd(),
            due_date: None,
            creditor: None,
        })
        .await
        .unwrap();

        let summary = svc.get_debt_summary().await.unwrap();
        assert_eq!(summary.debt_count, 2);
        assert_eq!(summary.total_balance_cents, 100000);
        assert_eq!(summary.total_minimum_payment_cents, 3000);
    }
}
