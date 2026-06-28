use async_trait::async_trait;
use chrono::Utc;
use phoenix_core::entities::{Debt, DebtType};
use phoenix_core::value_objects::{Currency, Percentage};
use rust_decimal::Decimal;
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

#[async_trait]
pub trait DebtRepository: Send + Sync {
    async fn create(&self, debt: Debt) -> anyhow::Result<Debt>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Debt>>;
    async fn update(&self, debt: Debt) -> anyhow::Result<Debt>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_active(&self) -> anyhow::Result<Vec<Debt>>;
    async fn list_by_type(&self, debt_type: DebtType) -> anyhow::Result<Vec<Debt>>;
}

#[derive(Debug, Clone)]
pub struct SqlDebtRepository {
    pool: DbPool,
}

impl SqlDebtRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn map_debt(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<Debt> {
    let dt_str: String = row.get("debt_type");
    let debt_type = match dt_str.as_str() {
        "credit_card" => DebtType::CreditCard,
        "student_loan" => DebtType::StudentLoan,
        "personal_loan" => DebtType::PersonalLoan,
        "medical" => DebtType::Medical,
        "auto" => DebtType::Auto,
        "other" => DebtType::Other,
        other => anyhow::bail!("invalid debt_type: {other}"),
    };
    let rate_str: String = row.get("interest_rate_pct");
    let interest_rate = Percentage::new(rate_str.parse::<Decimal>()?)?;
    Ok(Debt {
        id: row.get::<String, _>("id").parse()?,
        name: row.get("name"),
        debt_type,
        principal_cents: row.get("principal_cents"),
        remaining_balance_cents: row.get("remaining_balance_cents"),
        interest_rate,
        minimum_payment_cents: row.get("minimum_payment_cents"),
        currency: Currency::new(row.get::<String, _>("currency"))?,
        due_date: row
            .get::<Option<String>, _>("due_date")
            .map(|s| s.parse())
            .transpose()?,
        creditor: row.get("creditor"),
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn debt_type_str(d: &DebtType) -> &'static str {
    match d {
        DebtType::CreditCard => "credit_card",
        DebtType::StudentLoan => "student_loan",
        DebtType::PersonalLoan => "personal_loan",
        DebtType::Medical => "medical",
        DebtType::Auto => "auto",
        DebtType::Other => "other",
    }
}

#[async_trait]
impl DebtRepository for SqlDebtRepository {
    async fn create(&self, debt: Debt) -> anyhow::Result<Debt> {
        sqlx::query(
            "INSERT INTO debts (id, name, debt_type, principal_cents, remaining_balance_cents, interest_rate_pct, minimum_payment_cents, currency, due_date, creditor, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(debt.id.to_string())
        .bind(&debt.name)
        .bind(debt_type_str(&debt.debt_type))
        .bind(debt.principal_cents)
        .bind(debt.remaining_balance_cents)
        .bind(debt.interest_rate.value().to_string())
        .bind(debt.minimum_payment_cents)
        .bind(debt.currency.code())
        .bind(debt.due_date.map(|d| d.to_rfc3339()))
        .bind(&debt.creditor)
        .bind(debt.created_at.to_rfc3339())
        .bind(debt.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(debt.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created debt not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Debt>> {
        let row = sqlx::query("SELECT * FROM debts WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_debt).transpose()
    }

    async fn update(&self, debt: Debt) -> anyhow::Result<Debt> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE debts SET name = ?, debt_type = ?, principal_cents = ?, remaining_balance_cents = ?, interest_rate_pct = ?, minimum_payment_cents = ?, currency = ?, due_date = ?, creditor = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&debt.name)
        .bind(debt_type_str(&debt.debt_type))
        .bind(debt.principal_cents)
        .bind(debt.remaining_balance_cents)
        .bind(debt.interest_rate.value().to_string())
        .bind(debt.minimum_payment_cents)
        .bind(debt.currency.code())
        .bind(debt.due_date.map(|d| d.to_rfc3339()))
        .bind(&debt.creditor)
        .bind(&now)
        .bind(debt.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(debt.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated debt not found"))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM debts WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_active(&self) -> anyhow::Result<Vec<Debt>> {
        let rows =
            sqlx::query("SELECT * FROM debts WHERE remaining_balance_cents > 0 ORDER BY name")
                .fetch_all(&self.pool)
                .await?;
        rows.iter().map(map_debt).collect()
    }

    async fn list_by_type(&self, debt_type: DebtType) -> anyhow::Result<Vec<Debt>> {
        let rows = sqlx::query("SELECT * FROM debts WHERE debt_type = ? ORDER BY name")
            .bind(debt_type_str(&debt_type))
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_debt).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phoenix_core::value_objects::{Currency, Percentage};
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

    fn test_debt() -> Debt {
        Debt {
            id: Uuid::new_v4(),
            name: "Credit Card".into(),
            debt_type: DebtType::CreditCard,
            principal_cents: 100000,
            remaining_balance_cents: 50000,
            interest_rate: Percentage::new(Decimal::new(1899, 2)).unwrap(),
            minimum_payment_cents: 2500,
            currency: Currency::usd(),
            due_date: None,
            creditor: Some("Chase".into()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let pool = create_test_db().await;
        let repo = SqlDebtRepository::new(pool);
        let debt = test_debt();
        let created = repo.create(debt).await.unwrap();
        assert_eq!(created.name, "Credit Card");
        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_some());
    }

    #[tokio::test]
    async fn test_list_active() {
        let pool = create_test_db().await;
        let repo = SqlDebtRepository::new(pool);
        let debt = test_debt();
        repo.create(debt).await.unwrap();
        let mut paid = test_debt();
        paid.id = Uuid::new_v4();
        paid.remaining_balance_cents = 0;
        repo.create(paid).await.unwrap();
        let active = repo.list_active().await.unwrap();
        assert_eq!(active.len(), 1);
    }
}
