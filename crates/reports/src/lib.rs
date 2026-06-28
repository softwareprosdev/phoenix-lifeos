use chrono::{DateTime, Utc};
use phoenix_core::entities::{
    AccountType, BudgetPeriod, CreditReport, Goal, TransactionType,
};
use phoenix_db::repositories::{
    account_repo::{AccountRepository as _, SqlAccountRepository},
    budget_repo::{BudgetRepository as _, SqlBudgetRepository},
    credit_repo::{CreditReportRepository as _, SqlCreditReportRepository},
    debt_repo::{DebtRepository as _, SqlDebtRepository},
    goal_repo::{GoalRepository as _, SqlGoalRepository},
    investment_repo::{InvestmentRepository as _, SqlInvestmentRepository},
    transaction_repo::{SqlTransactionRepository, TransactionRepository as _},
};
use phoenix_db::DbPool;
use rust_decimal::Decimal;
use tracing::instrument;
use uuid::Uuid;

// ─── Report DTOs ─────────────────────────────────────────────────────────────

pub struct NetWorthReport {
    pub total_assets_cents: i64,
    pub total_liabilities_cents: i64,
    pub net_worth_cents: i64,
    pub account_count: usize,
    pub by_account_type: Vec<AccountTypeBalance>,
}

pub struct AccountTypeBalance {
    pub account_type: AccountType,
    pub total_cents: i64,
    pub count: usize,
}

pub struct CashFlowReport {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_income_cents: i64,
    pub total_expense_cents: i64,
    pub net_cents: i64,
    pub income_count: usize,
    pub expense_count: usize,
}

pub struct SpendingBreakdown {
    pub category_id: Uuid,
    pub category_name: String,
    pub total_cents: i64,
    pub percentage: f64,
    pub count: usize,
}

pub struct BudgetAnalysis {
    pub budget_id: Uuid,
    pub budget_name: String,
    pub total_limit_cents: i64,
    pub total_spent_cents: i64,
    pub remaining_cents: i64,
    pub utilization_pct: f64,
    pub period: BudgetPeriod,
    pub is_exceeded: bool,
}

pub struct DebtSummaryReport {
    pub total_balance_cents: i64,
    pub total_minimum_payment_cents: i64,
    pub debt_count: usize,
    pub average_apr_pct: Decimal,
    pub highest_apr_name: String,
    pub highest_apr_pct: Decimal,
    pub largest_debt_name: String,
    pub largest_debt_cents: i64,
}

pub struct GoalProgressReport {
    pub goals: Vec<Goal>,
    pub total_target_cents: i64,
    pub total_current_cents: i64,
    pub overall_progress_pct: f64,
    pub achieved_count: usize,
    pub active_count: usize,
}

pub struct InvestmentPerformanceReport {
    pub total_cost_basis_cents: i64,
    pub total_current_value_cents: i64,
    pub total_gain_loss_cents: i64,
    pub total_gain_loss_pct: f64,
    pub asset_count: usize,
    pub best_performer: String,
    pub worst_performer: String,
}

pub struct CreditScoreTrend {
    pub latest_score: i32,
    pub score_history: Vec<CreditReport>,
    pub score_change_3m: i32,
    pub score_change_6m: i32,
    pub score_change_12m: i32,
}

// ─── Service ─────────────────────────────────────────────────────────────────

pub struct ReportService {
    account_repo: SqlAccountRepository,
    tx_repo: SqlTransactionRepository,
    budget_repo: SqlBudgetRepository,
    debt_repo: SqlDebtRepository,
    goal_repo: SqlGoalRepository,
    investment_repo: SqlInvestmentRepository,
    credit_repo: SqlCreditReportRepository,
}

impl ReportService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            account_repo: SqlAccountRepository::new(pool.clone()),
            tx_repo: SqlTransactionRepository::new(pool.clone()),
            budget_repo: SqlBudgetRepository::new(pool.clone()),
            debt_repo: SqlDebtRepository::new(pool.clone()),
            goal_repo: SqlGoalRepository::new(pool.clone()),
            investment_repo: SqlInvestmentRepository::new(pool.clone()),
            credit_repo: SqlCreditReportRepository::new(pool),
        }
    }

    #[instrument(skip(self))]
    pub async fn get_net_worth_report(&self) -> anyhow::Result<NetWorthReport> {
        let accounts = self.account_repo.list_active().await?;
        let total_assets_cents: i64 = accounts
            .iter()
            .filter(|a| {
                matches!(
                    a.account_type,
                    AccountType::Checking
                        | AccountType::Savings
                        | AccountType::Cash
                        | AccountType::Investment
                        | AccountType::Business
                )
            })
            .map(|a| a.balance_cents)
            .sum();
        let total_liabilities_cents: i64 = accounts
            .iter()
            .filter(|a| {
                matches!(a.account_type, AccountType::Credit | AccountType::Loan)
            })
            .map(|a| a.balance_cents.abs())
            .sum();

        let mut by_type: std::collections::HashMap<AccountType, (i64, usize)> =
            std::collections::HashMap::new();
        for a in &accounts {
            let entry = by_type.entry(a.account_type.clone()).or_default();
            entry.0 += a.balance_cents;
            entry.1 += 1;
        }

        let by_account_type: Vec<AccountTypeBalance> = by_type
            .into_iter()
            .map(|(t, (total, count))| AccountTypeBalance {
                account_type: t,
                total_cents: total,
                count,
            })
            .collect();

        Ok(NetWorthReport {
            total_assets_cents,
            total_liabilities_cents,
            net_worth_cents: total_assets_cents - total_liabilities_cents,
            account_count: accounts.len(),
            by_account_type,
        })
    }

    #[instrument(skip(self))]
    pub async fn get_cash_flow_report(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> anyhow::Result<CashFlowReport> {
        let txs = self.tx_repo.list_by_date_range(start_date, end_date).await?;
        let total_income_cents: i64 = txs
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Income)
            .map(|t| t.amount_cents)
            .sum();
        let total_expense_cents: i64 = txs
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Expense)
            .map(|t| t.amount_cents)
            .sum();
        let income_count = txs
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Income)
            .count();
        let expense_count = txs
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Expense)
            .count();

        Ok(CashFlowReport {
            start_date,
            end_date,
            total_income_cents,
            total_expense_cents,
            net_cents: total_income_cents - total_expense_cents,
            income_count,
            expense_count,
        })
    }

    #[instrument(skip(self))]
    pub async fn get_spending_breakdown(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> anyhow::Result<Vec<SpendingBreakdown>> {
        let txs = self.tx_repo.list_by_date_range(start_date, end_date).await?;
        let expense_txs: Vec<_> = txs
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Expense)
            .collect();

        let total_expense: i64 = expense_txs.iter().map(|t| t.amount_cents).sum();

        let mut by_cat: std::collections::HashMap<Uuid, (i64, usize)> =
            std::collections::HashMap::new();
        for tx in &expense_txs {
            if let Some(cat_id) = tx.category_id {
                let entry = by_cat.entry(cat_id).or_default();
                entry.0 += tx.amount_cents;
                entry.1 += 1;
            }
        }

        let mut result = Vec::new();
        for (cat_id, (total_cents, count)) in by_cat {
            let category_name = self
                .account_repo
                .get_by_id(
                    expense_txs
                        .iter()
                        .find(|t| t.category_id == Some(cat_id))
                        .map(|t| t.account_id)
                        .unwrap_or_default(),
                )
                .await
                .ok()
                .and_then(|a| a.map(|_| "Category".into()))
                .unwrap_or_else(|| "Unknown".into());

            let percentage = if total_expense > 0 {
                (total_cents as f64 / total_expense as f64) * 100.0
            } else {
                0.0
            };

            result.push(SpendingBreakdown {
                category_id: cat_id,
                category_name,
                total_cents,
                percentage,
                count,
            });
        }

        result.sort_by(|a, b| b.total_cents.cmp(&a.total_cents));
        Ok(result)
    }

    #[instrument(skip(self))]
    pub async fn get_budget_analysis(&self, budget_id: Uuid) -> anyhow::Result<Option<BudgetAnalysis>> {
        let budget = self.budget_repo.get_by_id(budget_id).await?;
        match budget {
            None => Ok(None),
            Some(b) => {
                let txs = self.tx_repo.list_by_date_range(b.start_date, b.end_date).await?;
                let total_spent: i64 = txs
                    .iter()
                    .filter(|t| t.transaction_type == TransactionType::Expense)
                    .map(|t| t.amount_cents)
                    .sum();

                Ok(Some(BudgetAnalysis {
                    budget_id: b.id,
                    budget_name: b.name,
                    total_limit_cents: b.total_limit_cents,
                    total_spent_cents: total_spent,
                    remaining_cents: b.total_limit_cents - total_spent,
                    utilization_pct: if b.total_limit_cents > 0 {
                        (total_spent as f64 / b.total_limit_cents as f64) * 100.0
                    } else {
                        0.0
                    },
                    period: b.period,
                    is_exceeded: total_spent > b.total_limit_cents,
                }))
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn get_debt_summary_report(&self) -> anyhow::Result<DebtSummaryReport> {
        let debts = self.debt_repo.list_active().await?;
        let count = debts.len();
        let total_balance: i64 = debts.iter().map(|d| d.remaining_balance_cents).sum();
        let total_min_payment: i64 = debts.iter().map(|d| d.minimum_payment_cents).sum();

        let avg_apr = if count > 0 {
            let sum: Decimal = debts.iter().map(|d| d.interest_rate.value()).sum();
            sum / Decimal::from(count)
        } else {
            Decimal::ZERO
        };

        let highest_apr = debts
            .iter()
            .max_by_key(|d| d.interest_rate.value())
            .map(|d| (d.name.clone(), d.interest_rate.value()));
        let largest_debt = debts
            .iter()
            .max_by_key(|d| d.remaining_balance_cents)
            .map(|d| (d.name.clone(), d.remaining_balance_cents));

        Ok(DebtSummaryReport {
            total_balance_cents: total_balance,
            total_minimum_payment_cents: total_min_payment,
            debt_count: count,
            average_apr_pct: avg_apr,
            highest_apr_name: highest_apr.as_ref().map(|(n, _)| n.clone()).unwrap_or_default(),
            highest_apr_pct: highest_apr.map(|(_, p)| p).unwrap_or_default(),
            largest_debt_name: largest_debt.as_ref().map(|(n, _)| n.clone()).unwrap_or_default(),
            largest_debt_cents: largest_debt.map(|(_, c)| c).unwrap_or(0),
        })
    }

    #[instrument(skip(self))]
    pub async fn get_goal_progress_report(&self) -> anyhow::Result<GoalProgressReport> {
        let all_goals = self.goal_repo.list_active().await?;
        let achieved: Vec<Goal> = all_goals
            .iter()
            .filter(|g| g.is_achieved())
            .cloned()
            .collect();
        let active: Vec<Goal> = all_goals
            .iter()
            .filter(|g| !g.is_achieved())
            .cloned()
            .collect();

        let total_target: i64 = all_goals.iter().map(|g| g.target_amount_cents).sum();
        let total_current: i64 = all_goals.iter().map(|g| g.current_amount_cents).sum();
        let overall_progress = if total_target > 0 {
            (total_current as f64 / total_target as f64) * 100.0
        } else {
            100.0
        };

        Ok(GoalProgressReport {
            goals: all_goals,
            total_target_cents: total_target,
            total_current_cents: total_current,
            overall_progress_pct: overall_progress,
            achieved_count: achieved.len(),
            active_count: active.len(),
        })
    }

    #[instrument(skip(self))]
    pub async fn get_investment_performance_report(&self) -> anyhow::Result<InvestmentPerformanceReport> {
        // Fetch all investments from all accounts
        let accounts = self.account_repo.list_active().await?;
        let mut all_investments = Vec::new();
        for account in &accounts {
            if let Ok(invs) = self.investment_repo.list_by_account(account.id).await {
                all_investments.extend(invs);
            }
        }

        let total_cost: i64 = all_investments.iter().map(|i| i.cost_basis_cents).sum();
        let total_value: i64 = all_investments.iter().map(|i| i.current_value_cents).sum();
        let gain_loss = total_value - total_cost;
        let gain_loss_pct = if total_cost > 0 {
            (gain_loss as f64 / total_cost as f64) * 100.0
        } else {
            0.0
        };

        let best = all_investments
            .iter()
            .max_by_key(|i| i.current_value_cents - i.cost_basis_cents);
        let worst = all_investments
            .iter()
            .min_by_key(|i| i.current_value_cents - i.cost_basis_cents);

        Ok(InvestmentPerformanceReport {
            total_cost_basis_cents: total_cost,
            total_current_value_cents: total_value,
            total_gain_loss_cents: gain_loss,
            total_gain_loss_pct: gain_loss_pct,
            asset_count: all_investments.len(),
            best_performer: best.map(|i| i.name.clone()).unwrap_or_default(),
            worst_performer: worst.map(|i| i.name.clone()).unwrap_or_default(),
        })
    }

    #[instrument(skip(self))]
    pub async fn get_credit_score_trend(&self) -> anyhow::Result<CreditScoreTrend> {
        let now = Utc::now();
        let reports = self
            .credit_repo
            .list_by_date_range(DateTime::UNIX_EPOCH, now)
            .await?;

        let latest = reports.last().cloned();
        let score_3m_ago = reports
            .iter()
            .filter(|r| r.report_date >= now - chrono::Duration::days(90))
            .next()
            .map(|r| r.score);
        let score_6m_ago = reports
            .iter()
            .filter(|r| r.report_date >= now - chrono::Duration::days(180))
            .next()
            .map(|r| r.score);
        let score_12m_ago = reports
            .iter()
            .filter(|r| r.report_date >= now - chrono::Duration::days(365))
            .next()
            .map(|r| r.score);

        Ok(CreditScoreTrend {
            latest_score: latest.as_ref().map(|r| r.score).unwrap_or(0),
            score_history: reports,
            score_change_3m: latest
                .as_ref()
                .map(|l| l.score - score_3m_ago.unwrap_or(l.score))
                .unwrap_or(0),
            score_change_6m: latest
                .as_ref()
                .map(|l| l.score - score_6m_ago.unwrap_or(l.score))
                .unwrap_or(0),
            score_change_12m: latest
                .as_ref()
                .map(|l| l.score - score_12m_ago.unwrap_or(l.score))
                .unwrap_or(0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phoenix_core::entities::{Account, AccountType};
    use phoenix_core::value_objects::Currency;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query("PRAGMA foreign_keys = OFF")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY, name TEXT NOT NULL,
                account_type TEXT NOT NULL, institution TEXT,
                balance_cents INTEGER NOT NULL DEFAULT 0,
                currency TEXT NOT NULL DEFAULT 'USD',
                is_active INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY, account_id TEXT NOT NULL,
                amount_cents INTEGER NOT NULL, currency TEXT NOT NULL DEFAULT 'USD',
                transaction_type TEXT NOT NULL, category_id TEXT,
                merchant TEXT, description TEXT, date TEXT NOT NULL,
                tags TEXT NOT NULL DEFAULT '[]', is_recurring INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
            )",
        )
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
    async fn test_net_worth_report() {
        let pool = create_test_db().await;
        let svc = ReportService::new(pool.clone());

        // Add accounts directly via repo
        let account_repo = SqlAccountRepository::new(pool);
        let now = Utc::now();
        account_repo
            .create(Account {
                id: Uuid::new_v4(),
                name: "Checking".into(),
                account_type: AccountType::Checking,
                institution: None,
                balance_cents: 50000,
                currency: Currency::usd(),
                is_active: true,
                created_at: now,
                updated_at: now,
            })
            .await
            .unwrap();
        account_repo
            .create(Account {
                id: Uuid::new_v4(),
                name: "Credit Card".into(),
                account_type: AccountType::Credit,
                institution: None,
                balance_cents: -15000,
                currency: Currency::usd(),
                is_active: true,
                created_at: now,
                updated_at: now,
            })
            .await
            .unwrap();

        let report = svc.get_net_worth_report().await.unwrap();
        assert_eq!(report.total_assets_cents, 50000);
        assert_eq!(report.total_liabilities_cents, 15000);
        assert_eq!(report.net_worth_cents, 35000);
    }

    #[tokio::test]
    async fn test_cash_flow_report() {
        let pool = create_test_db().await;
        let svc = ReportService::new(pool.clone());
        let now = Utc::now();
        let aid = Uuid::new_v4();

        let tx_repo = SqlTransactionRepository::new(pool.clone());
        tx_repo
            .create(phoenix_core::entities::Transaction {
                id: Uuid::new_v4(),
                account_id: aid,
                amount_cents: 50000,
                currency: Currency::usd(),
                transaction_type: TransactionType::Income,
                category_id: None,
                merchant: None,
                description: Some("Salary".into()),
                date: now,
                tags: vec![],
                is_recurring: true,
                created_at: now,
                updated_at: now,
                deleted_at: None,
            })
            .await
            .unwrap();
        tx_repo
            .create(phoenix_core::entities::Transaction {
                id: Uuid::new_v4(),
                account_id: aid,
                amount_cents: 10000,
                currency: Currency::usd(),
                transaction_type: TransactionType::Expense,
                category_id: None,
                merchant: None,
                description: Some("Rent".into()),
                date: now,
                tags: vec![],
                is_recurring: true,
                created_at: now,
                updated_at: now,
                deleted_at: None,
            })
            .await
            .unwrap();

        let start = now - chrono::Duration::hours(1);
        let end = now + chrono::Duration::hours(1);
        let flow = svc.get_cash_flow_report(start, end).await.unwrap();
        assert_eq!(flow.total_income_cents, 50000);
        assert_eq!(flow.total_expense_cents, 10000);
    }

    #[tokio::test]
    async fn test_debt_summary_report() {
        let pool = create_test_db().await;
        let svc = ReportService::new(pool);
        let _debt_repo = SqlDebtRepository::new(
            SqlitePool::connect(":memory:").await.unwrap(),
        );
        let summary = svc.get_debt_summary_report().await.unwrap();
        assert_eq!(summary.debt_count, 0);
    }
}
