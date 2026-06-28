use async_trait::async_trait;
use chrono::Utc;
use phoenix_core::entities::{
    VaultAccessAction, VaultAccessLog, VaultItem, VaultItemType,
};
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

// ─── VaultItem ──────────────────────────────────────────────────────────────

#[async_trait]
pub trait VaultItemRepository: Send + Sync {
    async fn create(&self, item: VaultItem) -> anyhow::Result<VaultItem>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<VaultItem>>;
    async fn update(&self, item: VaultItem) -> anyhow::Result<VaultItem>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_by_type(&self, item_type: VaultItemType) -> anyhow::Result<Vec<VaultItem>>;
    async fn search(&self, query: &str) -> anyhow::Result<Vec<VaultItem>>;
}

fn map_vault_item(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<VaultItem> {
    let it_str: String = row.get("item_type");
    let item_type = match it_str.as_str() {
        "password" => VaultItemType::Password,
        "note" => VaultItemType::Note,
        "key" => VaultItemType::Key,
        "secret" => VaultItemType::Secret,
        other => anyhow::bail!("invalid vault item_type: {other}"),
    };
    Ok(VaultItem {
        id: row.get::<String, _>("id").parse()?,
        item_type,
        title: row.get("title"),
        encrypted_data: row.get("encrypted_data"),
        key_version: row.get("key_version"),
        metadata: row.get("metadata"),
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn vault_item_type_str(t: &VaultItemType) -> &'static str {
    match t {
        VaultItemType::Password => "password",
        VaultItemType::Note => "note",
        VaultItemType::Key => "key",
        VaultItemType::Secret => "secret",
    }
}

#[derive(Debug, Clone)]
pub struct SqlVaultItemRepository {
    pool: DbPool,
}

impl SqlVaultItemRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VaultItemRepository for SqlVaultItemRepository {
    async fn create(&self, item: VaultItem) -> anyhow::Result<VaultItem> {
        sqlx::query(
            "INSERT INTO vault_items (id, item_type, title, encrypted_data, key_version, metadata, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(item.id.to_string())
        .bind(vault_item_type_str(&item.item_type))
        .bind(&item.title)
        .bind(&item.encrypted_data)
        .bind(item.key_version)
        .bind(&item.metadata)
        .bind(item.created_at.to_rfc3339())
        .bind(item.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(item.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created vault item not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<VaultItem>> {
        let row = sqlx::query("SELECT * FROM vault_items WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_vault_item).transpose()
    }

    async fn update(&self, item: VaultItem) -> anyhow::Result<VaultItem> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE vault_items SET item_type = ?, title = ?, encrypted_data = ?, key_version = ?, metadata = ?, updated_at = ? WHERE id = ?",
        )
        .bind(vault_item_type_str(&item.item_type))
        .bind(&item.title)
        .bind(&item.encrypted_data)
        .bind(item.key_version)
        .bind(&item.metadata)
        .bind(&now)
        .bind(item.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(item.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated vault item not found"))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM vault_items WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_type(&self, item_type: VaultItemType) -> anyhow::Result<Vec<VaultItem>> {
        let rows =
            sqlx::query("SELECT * FROM vault_items WHERE item_type = ? ORDER BY title")
                .bind(vault_item_type_str(&item_type))
                .fetch_all(&self.pool)
                .await?;
        rows.iter().map(map_vault_item).collect()
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<VaultItem>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }
        let pattern = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let rows = sqlx::query(
            "SELECT * FROM vault_items WHERE title LIKE ? ESCAPE '\\' OR metadata LIKE ? ESCAPE '\\' ORDER BY title",
        )
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_vault_item).collect()
    }
}

// ─── VaultAccessLog ─────────────────────────────────────────────────────────

#[async_trait]
pub trait VaultAccessLogRepository: Send + Sync {
    async fn log_access(&self, log: VaultAccessLog) -> anyhow::Result<VaultAccessLog>;
    async fn list_by_item(&self, item_id: Uuid) -> anyhow::Result<Vec<VaultAccessLog>>;
    async fn list_recent(&self, limit: i64) -> anyhow::Result<Vec<VaultAccessLog>>;
}

fn map_vault_access_log(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<VaultAccessLog> {
    let action_str: String = row.get("action");
    let action = match action_str.as_str() {
        "view" => VaultAccessAction::View,
        "create" => VaultAccessAction::Create,
        "update" => VaultAccessAction::Update,
        "delete" => VaultAccessAction::Delete,
        other => anyhow::bail!("invalid vault access action: {other}"),
    };
    Ok(VaultAccessLog {
        id: row.get::<String, _>("id").parse()?,
        item_id: row.get::<String, _>("item_id").parse()?,
        action,
        created_at: row.get::<String, _>("created_at").parse()?,
    })
}

fn vault_access_action_str(a: &VaultAccessAction) -> &'static str {
    match a {
        VaultAccessAction::View => "view",
        VaultAccessAction::Create => "create",
        VaultAccessAction::Update => "update",
        VaultAccessAction::Delete => "delete",
    }
}

#[derive(Debug, Clone)]
pub struct SqlVaultAccessLogRepository {
    pool: DbPool,
}

impl SqlVaultAccessLogRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VaultAccessLogRepository for SqlVaultAccessLogRepository {
    async fn log_access(&self, log: VaultAccessLog) -> anyhow::Result<VaultAccessLog> {
        sqlx::query(
            "INSERT INTO vault_access_log (id, item_id, action, created_at) VALUES (?, ?, ?, ?)",
        )
        .bind(log.id.to_string())
        .bind(log.item_id.to_string())
        .bind(vault_access_action_str(&log.action))
        .bind(log.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(log)
    }

    async fn list_by_item(&self, item_id: Uuid) -> anyhow::Result<Vec<VaultAccessLog>> {
        let rows = sqlx::query(
            "SELECT * FROM vault_access_log WHERE item_id = ? ORDER BY created_at DESC",
        )
        .bind(item_id.to_string())
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_vault_access_log).collect()
    }

    async fn list_recent(&self, limit: i64) -> anyhow::Result<Vec<VaultAccessLog>> {
        let rows = sqlx::query(
            "SELECT * FROM vault_access_log ORDER BY created_at DESC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_vault_access_log).collect()
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
    async fn test_vault_item_crud() {
        let pool = create_test_db().await;
        let repo = SqlVaultItemRepository::new(pool);
        let item = VaultItem {
            id: Uuid::new_v4(),
            item_type: VaultItemType::Password,
            title: "Email Login".into(),
            encrypted_data: "base64encrypted".into(),
            key_version: 1,
            metadata: "{}".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let created = repo.create(item).await.unwrap();
        assert_eq!(created.title, "Email Login");
        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_some());
    }

    #[tokio::test]
    async fn test_access_log() {
        let pool = create_test_db().await;
        let repo = SqlVaultAccessLogRepository::new(pool);
        let log = VaultAccessLog {
            id: Uuid::new_v4(),
            item_id: Uuid::new_v4(),
            action: VaultAccessAction::View,
            created_at: Utc::now(),
        };
        let created = repo.log_access(log).await.unwrap();
        assert_eq!(created.action, VaultAccessAction::View);
        let recent = repo.list_recent(10).await.unwrap();
        assert_eq!(recent.len(), 1);
    }
}
