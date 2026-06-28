use chrono::{DateTime, Utc};
use phoenix_core::entities::CreditReport;
use phoenix_core::event_bus::{DomainEvent, EventBus};
use phoenix_db::repositories::credit_repo::{CreditReportRepository as _, SqlCreditReportRepository};
use phoenix_db::DbPool;
use rust_decimal::Decimal;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateCreditReportInput {
    pub score: i32,
    pub score_model: String,
    pub bureau: String,
    pub utilization_rate: Decimal,
    pub on_time_rate: Decimal,
    pub derogatory_marks: i32,
    pub total_accounts: i32,
    pub open_accounts: i32,
    pub hard_inquiries_12m: i32,
    pub report_date: DateTime<Utc>,
}

pub struct CreditScorePoint {
    pub date: DateTime<Utc>,
    pub score: i32,
    pub bureau: String,
}

pub struct UtilizationBreakdown {
    pub bureau: String,
    pub utilization_rate: Decimal,
    pub total_accounts: i32,
    pub open_accounts: i32,
}

pub struct CreditService {
    report_repo: SqlCreditReportRepository,
    event_bus: Option<EventBus>,
}

impl CreditService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            report_repo: SqlCreditReportRepository::new(pool),
            event_bus: None,
        }
    }

    pub fn with_event_bus(pool: DbPool, event_bus: EventBus) -> Self {
        Self {
            report_repo: SqlCreditReportRepository::new(pool),
            event_bus: Some(event_bus),
        }
    }

    #[instrument(skip(self))]
    pub async fn create_report(&self, input: CreateCreditReportInput) -> anyhow::Result<CreditReport> {
        if !(300..=850).contains(&input.score) {
            anyhow::bail!("credit score must be between 300 and 850");
        }
        let report = CreditReport {
            id: Uuid::new_v4(),
            score: input.score,
            score_model: input.score_model,
            bureau: input.bureau,
            utilization_rate: input.utilization_rate,
            on_time_rate: input.on_time_rate,
            derogatory_marks: input.derogatory_marks,
            total_accounts: input.total_accounts,
            open_accounts: input.open_accounts,
            hard_inquiries_12m: input.hard_inquiries_12m,
            report_date: input.report_date,
            created_at: Utc::now(),
        };
        let created = self.report_repo.create(report).await?;

        if let Some(ref bus) = self.event_bus {
            bus.publish(DomainEvent::CreditScoreUpdated {
                report_id: created.id,
                score: created.score,
                timestamp: Utc::now(),
            })
            .await;
        }

        Ok(created)
    }

    #[instrument(skip(self))]
    pub async fn get_latest_report(&self) -> anyhow::Result<Option<CreditReport>> {
        self.report_repo.latest().await
    }

    #[instrument(skip(self))]
    pub async fn list_reports(&self) -> anyhow::Result<Vec<CreditReport>> {
        self.report_repo
            .list_by_date_range(
                DateTime::UNIX_EPOCH,
                Utc::now() + chrono::Duration::days(1),
            )
            .await
    }

    #[instrument(skip(self))]
    pub async fn get_credit_score_history(&self) -> anyhow::Result<Vec<CreditScorePoint>> {
        let reports = self
            .report_repo
            .list_by_date_range(
                DateTime::UNIX_EPOCH,
                Utc::now() + chrono::Duration::days(1),
            )
            .await?;
        Ok(reports
            .into_iter()
            .map(|r| CreditScorePoint {
                date: r.report_date,
                score: r.score,
                bureau: r.bureau,
            })
            .collect())
    }

    #[instrument(skip(self))]
    pub async fn get_utilization_breakdown(&self) -> anyhow::Result<Vec<UtilizationBreakdown>> {
        let reports = self
            .report_repo
            .list_by_date_range(
                DateTime::UNIX_EPOCH,
                Utc::now() + chrono::Duration::days(1),
            )
            .await?;

        // Group latest report per bureau
        let mut latest_per_bureau: std::collections::HashMap<String, CreditReport> =
            std::collections::HashMap::new();
        for r in reports {
            let entry = latest_per_bureau.entry(r.bureau.clone());
            match entry {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    if r.report_date > o.get().report_date {
                        o.insert(r);
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(r);
                }
            }
        }

        Ok(latest_per_bureau
            .into_values()
            .map(|r| UtilizationBreakdown {
                bureau: r.bureau,
                utilization_rate: r.utilization_rate,
                total_accounts: r.total_accounts,
                open_accounts: r.open_accounts,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS credit_reports (
                id TEXT PRIMARY KEY, score INTEGER NOT NULL,
                score_model TEXT NOT NULL DEFAULT 'FICO_8',
                bureau TEXT NOT NULL,
                utilization_rate TEXT NOT NULL DEFAULT '0',
                on_time_rate TEXT NOT NULL DEFAULT '100',
                derogatory_marks INTEGER NOT NULL DEFAULT 0,
                total_accounts INTEGER NOT NULL DEFAULT 0,
                open_accounts INTEGER NOT NULL DEFAULT 0,
                hard_inquiries_12m INTEGER NOT NULL DEFAULT 0,
                report_date TEXT NOT NULL, created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_report() {
        let pool = create_test_db().await;
        let svc = CreditService::new(pool);
        let report = svc
            .create_report(CreateCreditReportInput {
                score: 720,
                score_model: "FICO_8".into(),
                bureau: "Experian".into(),
                utilization_rate: Decimal::new(25, 2),
                on_time_rate: Decimal::new(98, 1),
                derogatory_marks: 1,
                total_accounts: 12,
                open_accounts: 8,
                hard_inquiries_12m: 2,
                report_date: Utc::now(),
            })
            .await
            .unwrap();
        assert_eq!(report.score, 720);
    }

    #[tokio::test]
    async fn test_invalid_score() {
        let pool = create_test_db().await;
        let svc = CreditService::new(pool);
        let result = svc
            .create_report(CreateCreditReportInput {
                score: 999,
                score_model: "FICO_8".into(),
                bureau: "Exp".into(),
                utilization_rate: Decimal::ZERO,
                on_time_rate: Decimal::new(100, 1),
                derogatory_marks: 0,
                total_accounts: 1,
                open_accounts: 1,
                hard_inquiries_12m: 0,
                report_date: Utc::now(),
            })
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_latest_report() {
        let pool = create_test_db().await;
        let svc = CreditService::new(pool);
        svc.create_report(CreateCreditReportInput {
            score: 700,
            score_model: "FICO_8".into(),
            bureau: "TransUnion".into(),
            utilization_rate: Decimal::ZERO,
            on_time_rate: Decimal::new(100, 1),
            derogatory_marks: 0,
            total_accounts: 5,
            open_accounts: 3,
            hard_inquiries_12m: 1,
            report_date: Utc::now(),
        })
        .await
        .unwrap();

        let latest = svc.get_latest_report().await.unwrap();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().score, 700);
    }

    #[tokio::test]
    async fn test_score_history() {
        let pool = create_test_db().await;
        let svc = CreditService::new(pool);
        svc.create_report(CreateCreditReportInput {
            score: 680,
            score_model: "FICO_8".into(),
            bureau: "Experian".into(),
            utilization_rate: Decimal::new(30, 2),
            on_time_rate: Decimal::new(95, 1),
            derogatory_marks: 2,
            total_accounts: 10,
            open_accounts: 7,
            hard_inquiries_12m: 3,
            report_date: Utc::now() - chrono::Duration::days(30),
        })
        .await
        .unwrap();
        svc.create_report(CreateCreditReportInput {
            score: 720,
            score_model: "FICO_8".into(),
            bureau: "Experian".into(),
            utilization_rate: Decimal::new(25, 2),
            on_time_rate: Decimal::new(98, 1),
            derogatory_marks: 1,
            total_accounts: 12,
            open_accounts: 8,
            hard_inquiries_12m: 2,
            report_date: Utc::now(),
        })
        .await
        .unwrap();

        let history = svc.get_credit_score_history().await.unwrap();
        assert_eq!(history.len(), 2);
    }
}
