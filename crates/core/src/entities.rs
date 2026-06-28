use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_objects::{Currency, Money, Percentage};

// ─── Account ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Checking,
    Savings,
    Credit,
    Loan,
    Cash,
    Investment,
    Business,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub institution: Option<String>,
    pub balance_cents: i64,
    pub currency: Currency,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Account {
    pub fn balance(&self) -> Money {
        Money::from_cents(self.balance_cents, self.currency.clone())
    }
}

// ─── Transaction ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Income,
    Expense,
    Transfer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount_cents: i64,
    pub currency: Currency,
    pub transaction_type: TransactionType,
    pub category_id: Option<Uuid>,
    pub merchant: Option<String>,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub is_recurring: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Transaction {
    pub fn amount(&self) -> Money {
        Money::from_cents(self.amount_cents, self.currency.clone())
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

// ─── Budget ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetPeriod {
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub period: BudgetPeriod,
    pub total_limit_cents: i64,
    pub currency: Currency,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Budget {
    pub fn total_limit(&self) -> Money {
        Money::from_cents(self.total_limit_cents, self.currency.clone())
    }
}

// ─── Debt ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebtType {
    CreditCard,
    StudentLoan,
    PersonalLoan,
    Medical,
    Auto,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Debt {
    pub id: Uuid,
    pub name: String,
    pub debt_type: DebtType,
    pub principal_cents: i64,
    pub remaining_balance_cents: i64,
    pub interest_rate: Percentage,
    pub minimum_payment_cents: i64,
    pub currency: Currency,
    pub due_date: Option<DateTime<Utc>>,
    pub creditor: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Debt {
    pub fn remaining_balance(&self) -> Money {
        Money::from_cents(self.remaining_balance_cents, self.currency.clone())
    }

    pub fn minimum_payment(&self) -> Money {
        Money::from_cents(self.minimum_payment_cents, self.currency.clone())
    }

    pub fn monthly_interest_cents(&self) -> i64 {
        let monthly_rate = self.interest_rate.as_rate() / rust_decimal::Decimal::from(12);
        let interest = rust_decimal::Decimal::from(self.remaining_balance_cents) * monthly_rate;
        interest.round().try_into().unwrap_or(0)
    }
}

// ─── Goal ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalType {
    Savings,
    EmergencyFund,
    HomePurchase,
    Vehicle,
    Retirement,
    Education,
    Vacation,
    DebtPayoff,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub name: String,
    pub goal_type: GoalType,
    pub target_amount_cents: i64,
    pub current_amount_cents: i64,
    pub monthly_contribution_cents: i64,
    pub currency: Currency,
    pub deadline: Option<DateTime<Utc>>,
    pub linked_account_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Goal {
    pub fn target_amount(&self) -> Money {
        Money::from_cents(self.target_amount_cents, self.currency.clone())
    }

    pub fn current_amount(&self) -> Money {
        Money::from_cents(self.current_amount_cents, self.currency.clone())
    }

    pub fn remaining_amount_cents(&self) -> i64 {
        (self.target_amount_cents - self.current_amount_cents).max(0)
    }

    pub fn progress_percentage(&self) -> f64 {
        if self.target_amount_cents == 0 {
            return 100.0;
        }
        (self.current_amount_cents as f64 / self.target_amount_cents as f64 * 100.0).min(100.0)
    }

    pub fn is_achieved(&self) -> bool {
        self.current_amount_cents >= self.target_amount_cents
    }
}
