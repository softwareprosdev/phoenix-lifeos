use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid currency code: {0}")]
    InvalidCurrencyCode(String),

    #[error("Invalid money amount: {0}")]
    InvalidMoneyAmount(String),

    #[error("Currency mismatch: expected {expected}, got {got}")]
    CurrencyMismatch { expected: String, got: String },

    #[error("Invalid percentage: {0} (must be 0–100)")]
    InvalidPercentage(String),

    #[error("Invalid interest rate: {0}")]
    InvalidInterestRate(String),

    #[error("Negative balance is not allowed for account type {account_type}")]
    NegativeBalance { account_type: String },

    #[error("Budget limit exceeded: {category} has {spent} against limit of {limit}")]
    BudgetLimitExceeded { category: String, spent: String, limit: String },

    #[error("Debt balance cannot be negative")]
    DebtBelowZero,

    #[error("Goal target must be greater than zero")]
    InvalidGoalTarget,

    #[error("Validation error: {0}")]
    Validation(String),
}
