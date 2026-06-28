use chrono::Utc;
use phoenix_core::entities::{AssetType, Investment};
use phoenix_core::value_objects::Currency;
use phoenix_db::repositories::{
    investment_repo::{InvestmentRepository as _, SqlInvestmentRepository},
};
use phoenix_db::DbPool;
use rust_decimal::Decimal;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateInvestmentInput {
    pub account_id: Uuid,
    pub asset_type: AssetType,
    pub symbol: Option<String>,
    pub name: String,
    pub quantity: Decimal,
    pub cost_basis_cents: i64,
    pub current_value_cents: i64,
    pub currency: Currency,
}

#[derive(Debug)]
pub struct UpdateInvestmentInput {
    pub id: Uuid,
    pub account_id: Uuid,
    pub asset_type: AssetType,
    pub symbol: Option<String>,
    pub name: String,
    pub quantity: Decimal,
    pub cost_basis_cents: i64,
    pub current_value_cents: i64,
    pub currency: Currency,
}

pub struct PortfolioSummary {
    pub total_cost_basis_cents: i64,
    pub total_current_value_cents: i64,
    pub total_gain_loss_cents: i64,
    pub total_gain_loss_pct: f64,
    pub asset_count: usize,
    pub by_asset_type: Vec<AssetTypeBreakdown>,
}

pub struct AssetTypeBreakdown {
    pub asset_type: AssetType,
    pub cost_basis_cents: i64,
    pub current_value_cents: i64,
    pub gain_loss_cents: i64,
    pub count: usize,
}

pub struct InvestmentService {
    investment_repo: SqlInvestmentRepository,
}

impl InvestmentService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            investment_repo: SqlInvestmentRepository::new(pool),
        }
    }

    #[instrument(skip(self))]
    pub async fn create_investment(&self, input: CreateInvestmentInput) -> anyhow::Result<Investment> {
        let now = Utc::now();
        let investment = Investment {
            id: Uuid::new_v4(),
            account_id: input.account_id,
            asset_type: input.asset_type,
            symbol: input.symbol,
            name: input.name,
            quantity: input.quantity,
            cost_basis_cents: input.cost_basis_cents,
            current_value_cents: input.current_value_cents,
            currency: input.currency,
            created_at: now,
            updated_at: now,
        };
        self.investment_repo.create(investment).await
    }

    #[instrument(skip(self))]
    pub async fn get_investment(&self, id: Uuid) -> anyhow::Result<Option<Investment>> {
        self.investment_repo.get_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn update_investment(&self, input: UpdateInvestmentInput) -> anyhow::Result<Investment> {
        let investment = Investment {
            id: input.id,
            account_id: input.account_id,
            asset_type: input.asset_type,
            symbol: input.symbol,
            name: input.name,
            quantity: input.quantity,
            cost_basis_cents: input.cost_basis_cents,
            current_value_cents: input.current_value_cents,
            currency: input.currency,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.investment_repo.update(investment).await
    }

    #[instrument(skip(self))]
    pub async fn delete_investment(&self, id: Uuid) -> anyhow::Result<()> {
        self.investment_repo.delete(id).await
    }

    #[instrument(skip(self))]
    pub async fn list_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<Investment>> {
        self.investment_repo.list_by_account(account_id).await
    }

    #[instrument(skip(self))]
    pub async fn list_by_asset_type(&self, asset_type: AssetType) -> anyhow::Result<Vec<Investment>> {
        self.investment_repo.list_by_asset_type(asset_type).await
    }

    #[instrument(skip(self))]
    pub async fn get_portfolio_summary(&self) -> anyhow::Result<PortfolioSummary> {
        let mut all = Vec::new();
        // Collect from all asset types
        for at in &[
            AssetType::Stock,
            AssetType::Bond,
            AssetType::ETF,
            AssetType::MutualFund,
            AssetType::Crypto,
            AssetType::RealEstate,
            AssetType::Commodity,
            AssetType::Cash,
            AssetType::Other,
        ] {
            all.extend(self.investment_repo.list_by_asset_type(at.clone()).await?);
        }

        let total_cost: i64 = all.iter().map(|i| i.cost_basis_cents).sum();
        let total_value: i64 = all.iter().map(|i| i.current_value_cents).sum();
        let gain_loss = total_value - total_cost;
        let gain_loss_pct = if total_cost > 0 {
            (gain_loss as f64 / total_cost as f64) * 100.0
        } else {
            0.0
        };

        let mut by_type: std::collections::HashMap<AssetType, (i64, i64, usize)> =
            std::collections::HashMap::new();
        for inv in &all {
            let entry = by_type.entry(inv.asset_type.clone()).or_default();
            entry.0 += inv.cost_basis_cents;
            entry.1 += inv.current_value_cents;
            entry.2 += 1;
        }

        let by_asset_type: Vec<AssetTypeBreakdown> = by_type
            .into_iter()
            .map(|(asset_type, (cost, value, count))| AssetTypeBreakdown {
                asset_type,
                cost_basis_cents: cost,
                current_value_cents: value,
                gain_loss_cents: value - cost,
                count,
            })
            .collect();

        Ok(PortfolioSummary {
            total_cost_basis_cents: total_cost,
            total_current_value_cents: total_value,
            total_gain_loss_cents: gain_loss,
            total_gain_loss_pct: gain_loss_pct,
            asset_count: all.len(),
            by_asset_type,
        })
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

    fn test_investment(account_id: Uuid) -> CreateInvestmentInput {
        CreateInvestmentInput {
            account_id,
            asset_type: AssetType::Stock,
            symbol: Some("AAPL".into()),
            name: "Apple Inc.".into(),
            quantity: Decimal::new(10, 0),
            cost_basis_cents: 1500000,
            current_value_cents: 1750000,
            currency: Currency::usd(),
        }
    }

    #[tokio::test]
    async fn test_create_investment() {
        let pool = create_test_db().await;
        let svc = InvestmentService::new(pool);
        let aid = Uuid::new_v4();
        let inv = svc.create_investment(test_investment(aid)).await.unwrap();
        assert_eq!(inv.symbol.as_deref(), Some("AAPL"));
        assert_eq!(inv.current_value_cents, 1750000);
    }

    #[tokio::test]
    async fn test_portfolio_summary() {
        let pool = create_test_db().await;
        let svc = InvestmentService::new(pool);
        let aid = Uuid::new_v4();
        svc.create_investment(test_investment(aid)).await.unwrap();
        svc.create_investment(CreateInvestmentInput {
            account_id: aid,
            asset_type: AssetType::ETF,
            symbol: Some("VTI".into()),
            name: "Vanguard Total Stock".into(),
            quantity: Decimal::new(50, 0),
            cost_basis_cents: 5000000,
            current_value_cents: 5200000,
            currency: Currency::usd(),
        })
        .await
        .unwrap();

        let summary = svc.get_portfolio_summary().await.unwrap();
        assert_eq!(summary.asset_count, 2);
        assert_eq!(summary.total_cost_basis_cents, 6500000);
        assert_eq!(summary.total_current_value_cents, 6950000);
        assert_eq!(summary.by_asset_type.len(), 2);
    }

    #[tokio::test]
    async fn test_list_by_account() {
        let pool = create_test_db().await;
        let svc = InvestmentService::new(pool);
        let aid = Uuid::new_v4();
        svc.create_investment(test_investment(aid)).await.unwrap();

        let investments = svc.list_by_account(aid).await.unwrap();
        assert_eq!(investments.len(), 1);
    }
}
