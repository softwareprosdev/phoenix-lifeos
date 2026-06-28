use async_trait::async_trait;
use chrono::Utc;
use phoenix_core::entities::{AssetType, Investment};
use phoenix_core::value_objects::Currency;
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

#[async_trait]
pub trait InvestmentRepository: Send + Sync {
    async fn create(&self, investment: Investment) -> anyhow::Result<Investment>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Investment>>;
    async fn update(&self, investment: Investment) -> anyhow::Result<Investment>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<Investment>>;
    async fn list_by_asset_type(&self, asset_type: AssetType) -> anyhow::Result<Vec<Investment>>;
}

#[derive(Debug, Clone)]
pub struct SqlInvestmentRepository {
    pool: DbPool,
}

impl SqlInvestmentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn map_investment(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<Investment> {
    let at_str: String = row.get("asset_type");
    let asset_type = match at_str.as_str() {
        "stock" => AssetType::Stock,
        "bond" => AssetType::Bond,
        "etf" => AssetType::ETF,
        "mutual_fund" => AssetType::MutualFund,
        "crypto" => AssetType::Crypto,
        "real_estate" => AssetType::RealEstate,
        "commodity" => AssetType::Commodity,
        "cash" => AssetType::Cash,
        "other" => AssetType::Other,
        other => anyhow::bail!("invalid asset_type: {other}"),
    };
    Ok(Investment {
        id: row.get::<String, _>("id").parse()?,
        account_id: row.get::<String, _>("account_id").parse()?,
        asset_type,
        symbol: row.get("symbol"),
        name: row.get("name"),
        quantity: row.get::<String, _>("quantity").parse()?,
        cost_basis_cents: row.get("cost_basis_cents"),
        current_value_cents: row.get("current_value_cents"),
        currency: Currency::new(row.get::<String, _>("currency"))?,
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn asset_type_str(a: &AssetType) -> &'static str {
    match a {
        AssetType::Stock => "stock",
        AssetType::Bond => "bond",
        AssetType::ETF => "etf",
        AssetType::MutualFund => "mutual_fund",
        AssetType::Crypto => "crypto",
        AssetType::RealEstate => "real_estate",
        AssetType::Commodity => "commodity",
        AssetType::Cash => "cash",
        AssetType::Other => "other",
    }
}

#[async_trait]
impl InvestmentRepository for SqlInvestmentRepository {
    async fn create(&self, investment: Investment) -> anyhow::Result<Investment> {
        sqlx::query(
            "INSERT INTO investments (id, account_id, asset_type, symbol, name, quantity, cost_basis_cents, current_value_cents, currency, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(investment.id.to_string())
        .bind(investment.account_id.to_string())
        .bind(asset_type_str(&investment.asset_type))
        .bind(&investment.symbol)
        .bind(&investment.name)
        .bind(investment.quantity.to_string())
        .bind(investment.cost_basis_cents)
        .bind(investment.current_value_cents)
        .bind(investment.currency.code())
        .bind(investment.created_at.to_rfc3339())
        .bind(investment.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(investment.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created investment not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Investment>> {
        let row = sqlx::query("SELECT * FROM investments WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_investment).transpose()
    }

    async fn update(&self, investment: Investment) -> anyhow::Result<Investment> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE investments SET account_id = ?, asset_type = ?, symbol = ?, name = ?, quantity = ?, cost_basis_cents = ?, current_value_cents = ?, currency = ?, updated_at = ? WHERE id = ?",
        )
        .bind(investment.account_id.to_string())
        .bind(asset_type_str(&investment.asset_type))
        .bind(&investment.symbol)
        .bind(&investment.name)
        .bind(investment.quantity.to_string())
        .bind(investment.cost_basis_cents)
        .bind(investment.current_value_cents)
        .bind(investment.currency.code())
        .bind(&now)
        .bind(investment.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(investment.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated investment not found"))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM investments WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<Investment>> {
        let rows = sqlx::query("SELECT * FROM investments WHERE account_id = ? ORDER BY name")
            .bind(account_id.to_string())
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_investment).collect()
    }

    async fn list_by_asset_type(&self, asset_type: AssetType) -> anyhow::Result<Vec<Investment>> {
        let rows = sqlx::query("SELECT * FROM investments WHERE asset_type = ? ORDER BY name")
            .bind(asset_type_str(&asset_type))
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_investment).collect()
    }
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
            "CREATE TABLE IF NOT EXISTS investments (
                id TEXT PRIMARY KEY, account_id TEXT NOT NULL,
                asset_type TEXT NOT NULL, symbol TEXT, name TEXT NOT NULL,
                quantity TEXT NOT NULL,
                cost_basis_cents INTEGER NOT NULL,
                current_value_cents INTEGER NOT NULL,
                currency TEXT NOT NULL DEFAULT 'USD',
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let pool = create_test_db().await;
        let repo = SqlInvestmentRepository::new(pool);
        let investment = Investment {
            id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            asset_type: AssetType::Stock,
            symbol: Some("AAPL".into()),
            name: "Apple Inc.".into(),
            quantity: Decimal::new(10, 0),
            cost_basis_cents: 1500000,
            current_value_cents: 1750000,
            currency: Currency::usd(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let created = repo.create(investment).await.unwrap();
        assert_eq!(created.symbol.as_deref(), Some("AAPL"));
        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_some());
    }

    #[tokio::test]
    async fn test_list_by_asset_type() {
        let pool = create_test_db().await;
        let repo = SqlInvestmentRepository::new(pool);
        let inv = Investment {
            id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            asset_type: AssetType::Stock,
            symbol: Some("MSFT".into()),
            name: "Microsoft".into(),
            quantity: Decimal::new(5, 0),
            cost_basis_cents: 800000,
            current_value_cents: 950000,
            currency: Currency::usd(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repo.create(inv).await.unwrap();
        let stocks = repo.list_by_asset_type(AssetType::Stock).await.unwrap();
        assert_eq!(stocks.len(), 1);
    }
}
