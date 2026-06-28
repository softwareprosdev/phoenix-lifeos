use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::errors::DomainError;

// ─── Currency ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Currency(String);

impl Currency {
    pub fn new(code: impl Into<String>) -> Result<Self, DomainError> {
        let code = code.into().to_uppercase();
        if code.len() != 3 || !code.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(DomainError::InvalidCurrencyCode(code));
        }
        Ok(Self(code))
    }

    pub fn usd() -> Self {
        Self("USD".into())
    }

    pub fn code(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ─── Money ───────────────────────────────────────────────────────────────────

/// Represents a monetary amount with currency. Stored as integer cents internally
/// to avoid floating-point rounding errors in financial calculations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    /// Amount in the smallest currency unit (e.g. cents for USD)
    amount_cents: i64,
    currency: Currency,
}

impl Money {
    pub fn from_cents(amount_cents: i64, currency: Currency) -> Self {
        Self { amount_cents, currency }
    }

    pub fn from_decimal(amount: Decimal, currency: Currency) -> Result<Self, DomainError> {
        let cents = (amount * Decimal::from(100))
            .round()
            .try_into()
            .map_err(|_| DomainError::InvalidMoneyAmount(amount.to_string()))?;
        Ok(Self { amount_cents: cents, currency })
    }

    pub fn zero(currency: Currency) -> Self {
        Self { amount_cents: 0, currency }
    }

    pub fn amount_cents(&self) -> i64 {
        self.amount_cents
    }

    pub fn to_decimal(&self) -> Decimal {
        Decimal::from(self.amount_cents) / Decimal::from(100)
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn is_negative(&self) -> bool {
        self.amount_cents < 0
    }

    pub fn is_zero(&self) -> bool {
        self.amount_cents == 0
    }

    pub fn abs(&self) -> Self {
        Self { amount_cents: self.amount_cents.abs(), currency: self.currency.clone() }
    }

    pub fn add(&self, other: &Money) -> Result<Money, DomainError> {
        self.assert_same_currency(other)?;
        Ok(Self {
            amount_cents: self.amount_cents + other.amount_cents,
            currency: self.currency.clone(),
        })
    }

    pub fn subtract(&self, other: &Money) -> Result<Money, DomainError> {
        self.assert_same_currency(other)?;
        Ok(Self {
            amount_cents: self.amount_cents - other.amount_cents,
            currency: self.currency.clone(),
        })
    }

    pub fn multiply(&self, factor: Decimal) -> Money {
        let new_cents = (Decimal::from(self.amount_cents) * factor).round();
        Self {
            amount_cents: new_cents.try_into().unwrap_or(i64::MAX),
            currency: self.currency.clone(),
        }
    }

    fn assert_same_currency(&self, other: &Money) -> Result<(), DomainError> {
        if self.currency != other.currency {
            return Err(DomainError::CurrencyMismatch {
                expected: self.currency.code().to_string(),
                got: other.currency.code().to_string(),
            });
        }
        Ok(())
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} {}", self.to_decimal(), self.currency)
    }
}

// ─── Percentage ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Percentage(Decimal);

impl Percentage {
    pub fn new(value: Decimal) -> Result<Self, DomainError> {
        if value < Decimal::ZERO || value > Decimal::from(100) {
            return Err(DomainError::InvalidPercentage(value.to_string()));
        }
        Ok(Self(value))
    }

    pub fn from_rate(rate: Decimal) -> Result<Self, DomainError> {
        Self::new(rate * Decimal::from(100))
    }

    pub fn value(&self) -> Decimal {
        self.0
    }

    pub fn as_rate(&self) -> Decimal {
        self.0 / Decimal::from(100)
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.0)
    }
}
