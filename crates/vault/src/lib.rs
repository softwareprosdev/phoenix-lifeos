use chrono::Utc;
use phoenix_core::entities::{VaultAccessAction, VaultAccessLog, VaultItem, VaultItemType};
use phoenix_core::event_bus::{DomainEvent, EventBus};
use phoenix_db::repositories::vault_repo::{
    VaultAccessLogRepository as _, VaultItemRepository as _, SqlVaultAccessLogRepository,
    SqlVaultItemRepository,
};
use phoenix_db::DbPool;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateVaultItemInput {
    pub item_type: VaultItemType,
    pub title: String,
    pub encrypted_data: String,
    pub key_version: i32,
    pub metadata: String,
}

#[derive(Debug)]
pub struct UpdateVaultItemInput {
    pub id: Uuid,
    pub item_type: VaultItemType,
    pub title: String,
    pub encrypted_data: String,
    pub key_version: i32,
    pub metadata: String,
}

pub struct VaultService {
    item_repo: SqlVaultItemRepository,
    access_log_repo: SqlVaultAccessLogRepository,
    event_bus: Option<EventBus>,
}

impl VaultService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            item_repo: SqlVaultItemRepository::new(pool.clone()),
            access_log_repo: SqlVaultAccessLogRepository::new(pool),
            event_bus: None,
        }
    }

    pub fn with_event_bus(pool: DbPool, event_bus: EventBus) -> Self {
        Self {
            item_repo: SqlVaultItemRepository::new(pool.clone()),
            access_log_repo: SqlVaultAccessLogRepository::new(pool),
            event_bus: Some(event_bus),
        }
    }

    #[instrument(skip(self))]
    pub async fn create_item(&self, input: CreateVaultItemInput) -> anyhow::Result<VaultItem> {
        let now = Utc::now();
        let item = VaultItem {
            id: Uuid::new_v4(),
            item_type: input.item_type,
            title: input.title,
            encrypted_data: input.encrypted_data,
            key_version: input.key_version,
            metadata: input.metadata,
            created_at: now,
            updated_at: now,
        };
        let created = self.item_repo.create(item).await?;

        self.log_access(created.id, VaultAccessAction::Create).await;

        if let Some(ref bus) = self.event_bus {
            bus.publish(DomainEvent::VaultItemAccessed {
                item_id: created.id,
                action: "create".into(),
                timestamp: Utc::now(),
            })
            .await;
        }

        Ok(created)
    }

    #[instrument(skip(self))]
    pub async fn get_item(&self, id: Uuid) -> anyhow::Result<Option<VaultItem>> {
        let item = self.item_repo.get_by_id(id).await?;
        if item.is_some() {
            self.log_access(id, VaultAccessAction::View).await;
            if let Some(ref bus) = self.event_bus {
                bus.publish(DomainEvent::VaultItemAccessed {
                    item_id: id,
                    action: "view".into(),
                    timestamp: Utc::now(),
                })
                .await;
            }
        }
        Ok(item)
    }

    #[instrument(skip(self))]
    pub async fn update_item(&self, input: UpdateVaultItemInput) -> anyhow::Result<VaultItem> {
        let now = Utc::now();
        let item = VaultItem {
            id: input.id,
            item_type: input.item_type,
            title: input.title,
            encrypted_data: input.encrypted_data,
            key_version: input.key_version,
            metadata: input.metadata,
            created_at: now,
            updated_at: now,
        };
        let updated = self.item_repo.update(item).await?;

        self.log_access(updated.id, VaultAccessAction::Update).await;

        if let Some(ref bus) = self.event_bus {
            bus.publish(DomainEvent::VaultItemAccessed {
                item_id: updated.id,
                action: "update".into(),
                timestamp: Utc::now(),
            })
            .await;
        }

        Ok(updated)
    }

    #[instrument(skip(self))]
    pub async fn delete_item(&self, id: Uuid) -> anyhow::Result<()> {
        self.log_access(id, VaultAccessAction::Delete).await;

        if let Some(ref bus) = self.event_bus {
            bus.publish(DomainEvent::VaultItemAccessed {
                item_id: id,
                action: "delete".into(),
                timestamp: Utc::now(),
            })
            .await;
        }

        self.item_repo.delete(id).await
    }

    #[instrument(skip(self))]
    pub async fn list_by_type(&self, item_type: VaultItemType) -> anyhow::Result<Vec<VaultItem>> {
        self.item_repo.list_by_type(item_type).await
    }

    #[instrument(skip(self))]
    pub async fn search(&self, query: &str) -> anyhow::Result<Vec<VaultItem>> {
        self.item_repo.search(query).await
    }

    #[instrument(skip(self))]
    pub async fn get_access_log(&self, item_id: Uuid) -> anyhow::Result<Vec<VaultAccessLog>> {
        self.access_log_repo.list_by_item(item_id).await
    }

    #[instrument(skip(self))]
    pub async fn get_recent_access_logs(&self, limit: i64) -> anyhow::Result<Vec<VaultAccessLog>> {
        self.access_log_repo.list_recent(limit).await
    }

    async fn log_access(&self, item_id: Uuid, action: VaultAccessAction) {
        let log = VaultAccessLog {
            id: Uuid::new_v4(),
            item_id,
            action,
            created_at: Utc::now(),
        };
        self.access_log_repo.log_access(log).await.ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS vault_items (
                id TEXT PRIMARY KEY, item_type TEXT NOT NULL,
                title TEXT NOT NULL, encrypted_data TEXT NOT NULL,
                key_version INTEGER NOT NULL DEFAULT 1,
                metadata TEXT NOT NULL DEFAULT '{}',
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS vault_access_log (
                id TEXT PRIMARY KEY, item_id TEXT NOT NULL,
                action TEXT NOT NULL, created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_vault_item() {
        let pool = create_test_db().await;
        let svc = VaultService::new(pool);
        let item = svc
            .create_item(CreateVaultItemInput {
                item_type: VaultItemType::Password,
                title: "Email Login".into(),
                encrypted_data: "base64encrypted".into(),
                key_version: 1,
                metadata: "{}".into(),
            })
            .await
            .unwrap();
        assert_eq!(item.title, "Email Login");

        let fetched = svc.get_item(item.id).await.unwrap();
        assert!(fetched.is_some());
    }

    #[tokio::test]
    async fn test_access_logging() {
        let pool = create_test_db().await;
        let svc = VaultService::new(pool);
        let item = svc
            .create_item(CreateVaultItemInput {
                item_type: VaultItemType::Note,
                title: "Secret Note".into(),
                encrypted_data: "encrypted".into(),
                key_version: 1,
                metadata: "{}".into(),
            })
            .await
            .unwrap();

        let logs = svc.get_access_log(item.id).await.unwrap();
        assert!(!logs.is_empty());
        assert_eq!(logs[0].action, VaultAccessAction::Create);
    }

    #[tokio::test]
    async fn test_search() {
        let pool = create_test_db().await;
        let svc = VaultService::new(pool);
        svc.create_item(CreateVaultItemInput {
            item_type: VaultItemType::Password,
            title: "Bank Login".into(),
            encrypted_data: "enc".into(),
            key_version: 1,
            metadata: "{}".into(),
        })
        .await
        .unwrap();

        let results = svc.search("Bank").await.unwrap();
        assert_eq!(results.len(), 1);
    }
}
