use phoenix_core::entities::{Account, Transaction};
use uuid::Uuid;

/// Repository trait for Account persistence.
#[async_trait::async_trait]
pub trait AccountRepository: Send + Sync {
    async fn create(&self, account: Account) -> anyhow::Result<Account>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Account>>;
    async fn list_active(&self) -> anyhow::Result<Vec<Account>>;
    async fn update(&self, account: Account) -> anyhow::Result<Account>;
    async fn deactivate(&self, id: Uuid) -> anyhow::Result<()>;
}

/// Repository trait for Transaction persistence.
#[async_trait::async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn create(&self, tx: Transaction) -> anyhow::Result<Transaction>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Transaction>>;
    async fn list_by_account(&self, account_id: Uuid) -> anyhow::Result<Vec<Transaction>>;
    async fn soft_delete(&self, id: Uuid) -> anyhow::Result<()>;
}
