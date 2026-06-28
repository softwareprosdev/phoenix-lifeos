use async_trait::async_trait;
use chrono::Utc;
use phoenix_core::entities::{Account, AccountType};
use phoenix_core::value_objects::Currency;
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn create(&self, account: Account) -> anyhow::Result<Account>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Account>>;
    async fn list_active(&self) -> anyhow::Result<Vec<Account>>;
    async fn update(&self, account: Account) -> anyhow::Result<Account>;
    async fn deactivate(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct SqlAccountRepository {
    pool: DbPool,
}

impl SqlAccountRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn map_account(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<Account> {
    let account_type_str: String = row.get("account_type");
    let account_type = match account_type_str.as_str() {
        "checking" => AccountType::Checking,
        "savings" => AccountType::Savings,
        "credit" => AccountType::Credit,
        "loan" => AccountType::Loan,
        "cash" => AccountType::Cash,
        "investment" => AccountType::Investment,
        "business" => AccountType::Business,
        other => anyhow::bail!("invalid account_type: {other}"),
    };
    Ok(Account {
        id: row.get::<String, _>("id").parse()?,
        name: row.get("name"),
        account_type,
        institution: row.get("institution"),
        balance_cents: row.get("balance_cents"),
        currency: Currency::new(row.get::<String, _>("currency"))?,
        is_active: row.get("is_active"),
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn account_type_str(t: &AccountType) -> &'static str {
    match t {
        AccountType::Checking => "checking",
        AccountType::Savings => "savings",
        AccountType::Credit => "credit",
        AccountType::Loan => "loan",
        AccountType::Cash => "cash",
        AccountType::Investment => "investment",
        AccountType::Business => "business",
    }
}

#[async_trait]
impl AccountRepository for SqlAccountRepository {
    async fn create(&self, account: Account) -> anyhow::Result<Account> {
        sqlx::query(
            "INSERT INTO accounts (id, name, account_type, institution, balance_cents, currency, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(account.id.to_string())
        .bind(&account.name)
        .bind(account_type_str(&account.account_type))
        .bind(&account.institution)
        .bind(account.balance_cents)
        .bind(account.currency.code())
        .bind(account.is_active)
        .bind(account.created_at.to_rfc3339())
        .bind(account.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(account.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created account not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Account>> {
        let row = sqlx::query("SELECT * FROM accounts WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_account).transpose()
    }

    async fn list_active(&self) -> anyhow::Result<Vec<Account>> {
        let rows = sqlx::query("SELECT * FROM accounts WHERE is_active = 1")
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_account).collect()
    }

    async fn update(&self, account: Account) -> anyhow::Result<Account> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE accounts SET name = ?, account_type = ?, institution = ?, balance_cents = ?, currency = ?, is_active = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&account.name)
        .bind(account_type_str(&account.account_type))
        .bind(&account.institution)
        .bind(account.balance_cents)
        .bind(account.currency.code())
        .bind(account.is_active)
        .bind(&now)
        .bind(account.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(account.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated account not found"))
    }

    async fn deactivate(&self, id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE accounts SET is_active = 0, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phoenix_core::value_objects::Currency;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
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
        pool
    }

    fn test_account() -> Account {
        Account {
            id: Uuid::new_v4(),
            name: "Test Checking".into(),
            account_type: AccountType::Checking,
            institution: Some("Test Bank".into()),
            balance_cents: 10000,
            currency: Currency::usd(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let pool = create_test_db().await;
        let repo = SqlAccountRepository::new(pool);
        let account = test_account();
        let created = repo.create(account).await.unwrap();
        assert_eq!(created.name, "Test Checking");
        assert!(created.is_active);
        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().name, "Test Checking");
    }

    #[tokio::test]
    async fn test_list_active() {
        let pool = create_test_db().await;
        let repo = SqlAccountRepository::new(pool);
        let a1 = test_account();
        repo.create(a1).await.unwrap();
        let mut a2 = test_account();
        a2.id = Uuid::new_v4();
        a2.is_active = false;
        repo.create(a2).await.unwrap();
        let active = repo.list_active().await.unwrap();
        assert_eq!(active.len(), 1);
    }

    #[tokio::test]
    async fn test_update() {
        let pool = create_test_db().await;
        let repo = SqlAccountRepository::new(pool);
        let account = test_account();
        let created = repo.create(account).await.unwrap();
        let mut updated = created;
        updated.name = "Updated Name".into();
        let result = repo.update(updated).await.unwrap();
        assert_eq!(result.name, "Updated Name");
    }

    #[tokio::test]
    async fn test_deactivate() {
        let pool = create_test_db().await;
        let repo = SqlAccountRepository::new(pool);
        let account = test_account();
        let created = repo.create(account).await.unwrap();
        repo.deactivate(created.id).await.unwrap();
        let fetched = repo.get_by_id(created.id).await.unwrap().unwrap();
        assert!(!fetched.is_active);
    }
}
