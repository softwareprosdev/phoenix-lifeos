use async_trait::async_trait;
use chrono::{DateTime, Utc};
use phoenix_core::entities::CreditReport;
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

#[async_trait]
pub trait CreditReportRepository: Send + Sync {
    async fn create(&self, report: CreditReport) -> anyhow::Result<CreditReport>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<CreditReport>>;
    async fn update(&self, report: CreditReport) -> anyhow::Result<CreditReport>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<CreditReport>>;
    async fn latest(&self) -> anyhow::Result<Option<CreditReport>>;
}

#[derive(Debug, Clone)]
pub struct SqlCreditReportRepository {
    pool: DbPool,
}

impl SqlCreditReportRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn map_credit_report(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<CreditReport> {
    Ok(CreditReport {
        id: row.get::<String, _>("id").parse()?,
        score: row.get("score"),
        score_model: row.get("score_model"),
        bureau: row.get("bureau"),
        utilization_rate: row.get::<String, _>("utilization_rate").parse()?,
        on_time_rate: row.get::<String, _>("on_time_rate").parse()?,
        derogatory_marks: row.get("derogatory_marks"),
        total_accounts: row.get("total_accounts"),
        open_accounts: row.get("open_accounts"),
        hard_inquiries_12m: row.get("hard_inquiries_12m"),
        report_date: row.get::<String, _>("report_date").parse()?,
        created_at: row.get::<String, _>("created_at").parse()?,
    })
}

#[async_trait]
impl CreditReportRepository for SqlCreditReportRepository {
    async fn create(&self, report: CreditReport) -> anyhow::Result<CreditReport> {
        sqlx::query(
            "INSERT INTO credit_reports (id, score, score_model, bureau, utilization_rate, on_time_rate, derogatory_marks, total_accounts, open_accounts, hard_inquiries_12m, report_date, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(report.id.to_string())
        .bind(report.score)
        .bind(&report.score_model)
        .bind(&report.bureau)
        .bind(report.utilization_rate.to_string())
        .bind(report.on_time_rate.to_string())
        .bind(report.derogatory_marks)
        .bind(report.total_accounts)
        .bind(report.open_accounts)
        .bind(report.hard_inquiries_12m)
        .bind(report.report_date.to_rfc3339())
        .bind(report.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(report.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created credit report not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<CreditReport>> {
        let row = sqlx::query("SELECT * FROM credit_reports WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_credit_report).transpose()
    }

    async fn update(&self, report: CreditReport) -> anyhow::Result<CreditReport> {
        sqlx::query(
            "UPDATE credit_reports SET score = ?, score_model = ?, bureau = ?, utilization_rate = ?, on_time_rate = ?, derogatory_marks = ?, total_accounts = ?, open_accounts = ?, hard_inquiries_12m = ?, report_date = ? WHERE id = ?",
        )
        .bind(report.score)
        .bind(&report.score_model)
        .bind(&report.bureau)
        .bind(report.utilization_rate.to_string())
        .bind(report.on_time_rate.to_string())
        .bind(report.derogatory_marks)
        .bind(report.total_accounts)
        .bind(report.open_accounts)
        .bind(report.hard_inquiries_12m)
        .bind(report.report_date.to_rfc3339())
        .bind(report.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(report.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated credit report not found"))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM credit_reports WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<CreditReport>> {
        let rows = sqlx::query(
            "SELECT * FROM credit_reports WHERE report_date >= ? AND report_date <= ? ORDER BY report_date",
        )
        .bind(start.to_rfc3339())
        .bind(end.to_rfc3339())
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_credit_report).collect()
    }

    async fn latest(&self) -> anyhow::Result<Option<CreditReport>> {
        let row = sqlx::query(
            "SELECT * FROM credit_reports ORDER BY report_date DESC LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await?;
        row.as_ref().map(map_credit_report).transpose()
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
    async fn test_create_and_latest() {
        let pool = create_test_db().await;
        let repo = SqlCreditReportRepository::new(pool);
        let report = CreditReport {
            id: Uuid::new_v4(),
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
            created_at: Utc::now(),
        };
        repo.create(report).await.unwrap();
        let latest = repo.latest().await.unwrap();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().score, 720);
    }

    #[tokio::test]
    async fn test_list_by_date_range() {
        let pool = create_test_db().await;
        let repo = SqlCreditReportRepository::new(pool);
        let report = CreditReport {
            id: Uuid::new_v4(),
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
            created_at: Utc::now(),
        };
        repo.create(report).await.unwrap();
        let start = Utc::now() - chrono::Duration::hours(1);
        let end = Utc::now() + chrono::Duration::hours(1);
        let reports = repo.list_by_date_range(start, end).await.unwrap();
        assert!(!reports.is_empty());
    }
}
