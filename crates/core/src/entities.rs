use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_objects::{Currency, Money, Percentage};

// ─── Account ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

// ─── Category ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CategoryType {
    Income,
    Expense,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub category_type: CategoryType,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// ─── BudgetItem ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetItem {
    pub id: Uuid,
    pub budget_id: Uuid,
    pub category_id: Option<Uuid>,
    pub limit_cents: i64,
    pub spent_cents: i64,
}

// ─── CreditReport ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditReport {
    pub id: Uuid,
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
    pub created_at: DateTime<Utc>,
}

// ─── Credit Repair ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditAccount {
    pub id: Uuid,
    pub name: String,
    pub issuer: String,
    pub account_type: String,
    pub credit_limit_cents: i64,
    pub balance_cents: i64,
    pub apr: String,
    pub payment_status: String,
    pub opened_date: Option<DateTime<Utc>>,
    pub closed_date: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditInquiry {
    pub id: Uuid,
    pub bureau: String,
    pub creditor_name: String,
    pub inquiry_type: String,
    pub inquiry_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditDerogatory {
    pub id: Uuid,
    pub creditor_name: String,
    pub account_number_encrypted: Option<String>,
    pub item_type: String,
    pub amount_cents: i64,
    pub original_amount_cents: i64,
    pub status: String,
    pub opened_date: Option<DateTime<Utc>>,
    pub expected_removal_date: Option<DateTime<Utc>>,
    pub is_disputed: bool,
    pub bureau: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeLetterTemplate {
    pub id: String,
    pub template_code: String,
    pub name: String,
    pub description: String,
    pub legal_basis: Option<String>,
    pub recipient_type: String,
    pub body_template: String,
    pub variable_schema: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegativeItem {
    pub id: Uuid,
    pub creditor_name: String,
    pub item_type: String,
    pub bureau: String,
    pub account_number_encrypted: Option<String>,
    pub amount_cents: i64,
    pub expected_removal_date: Option<DateTime<Utc>>,
    pub is_disputed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeCase {
    pub id: Uuid,
    pub negative_item_id: Option<Uuid>,
    pub dispute_type: String,
    pub status: String,
    pub status_basis: Option<String>,
    pub bureau: String,
    pub statute_basis: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeLetter {
    pub id: Uuid,
    pub dispute_case_id: Uuid,
    pub template_id: Option<String>,
    pub recipient_type: String,
    pub recipient_name: String,
    pub recipient_address: Option<String>,
    pub generated_content_encrypted: Option<String>,
    pub tracking_number: Option<String>,
    pub sent_date: Option<DateTime<Utc>>,
    pub response_received_date: Option<DateTime<Utc>>,
    pub response_summary: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeTimelineEntry {
    pub id: Uuid,
    pub dispute_case_id: Uuid,
    pub event_type: String,
    pub description: String,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessCreditProfile {
    pub id: Uuid,
    pub business_name: String,
    pub ein_encrypted: Option<String>,
    pub duns_number: Option<String>,
    pub paydex_score: Option<i32>,
    pub business_credit_score: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorCreditAccount {
    pub id: Uuid,
    pub business_credit_profile_id: Uuid,
    pub vendor_name: String,
    pub account_type: String,
    pub credit_limit_cents: i64,
    pub payment_status: String,
    pub reports_to_dnb: bool,
    pub reports_to_experian: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── Investment ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Stock,
    Bond,
    ETF,
    MutualFund,
    Crypto,
    RealEstate,
    Commodity,
    Cash,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Investment {
    pub id: Uuid,
    pub account_id: Uuid,
    pub asset_type: AssetType,
    pub symbol: Option<String>,
    pub name: String,
    pub quantity: Decimal,
    pub cost_basis_cents: i64,
    pub current_value_cents: i64,
    pub currency: Currency,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── Vault ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VaultItemType {
    Password,
    Note,
    Key,
    Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultItem {
    pub id: Uuid,
    pub item_type: VaultItemType,
    pub title: String,
    pub encrypted_data: String,
    pub key_version: i32,
    pub metadata: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VaultAccessAction {
    View,
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultAccessLog {
    pub id: Uuid,
    pub item_id: Uuid,
    pub action: VaultAccessAction,
    pub created_at: DateTime<Utc>,
}

// ─── Document ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentSource {
    Upload,
    Email,
    Scan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub file_type: String,
    pub mime_type: Option<String>,
    pub encrypted_path: String,
    pub ocr_text_encrypted: Option<String>,
    pub extracted_metadata: String,
    pub tags: Vec<String>,
    pub source: DocumentSource,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── CRM ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub organization: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionType {
    Call,
    Email,
    Meeting,
    Note,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInteraction {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub interaction_type: InteractionType,
    pub subject: Option<String>,
    pub notes: Option<String>,
    pub date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// ─── Event ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub event_type: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
}
