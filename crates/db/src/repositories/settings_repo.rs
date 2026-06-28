use async_trait::async_trait;
use chrono::Utc;
use sqlx::Row;
use std::collections::HashMap;

use crate::DbPool;

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn get(&self, key: &str) -> anyhow::Result<Option<String>>;
    async fn set(&self, key: &str, value: &str) -> anyhow::Result<()>;
    async fn get_all(&self) -> anyhow::Result<HashMap<String, String>>;
}

#[derive(Debug, Clone)]
pub struct SqlSettingsRepository {
    pool: DbPool,
}

impl SqlSettingsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsRepository for SqlSettingsRepository {
    async fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        let row = sqlx::query("SELECT value FROM app_settings WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.get::<String, _>("value")))
    }

    async fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO app_settings (key, value, updated_at) VALUES (?, ?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        )
        .bind(key)
        .bind(value)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_all(&self) -> anyhow::Result<HashMap<String, String>> {
        let rows = sqlx::query("SELECT key, value FROM app_settings")
            .fetch_all(&self.pool)
            .await?;
        let mut map = HashMap::new();
        for row in rows {
            map.insert(row.get("key"), row.get("value"));
        }
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS app_settings (
                key TEXT PRIMARY KEY, value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_set_and_get() {
        let pool = create_test_db().await;
        let repo = SqlSettingsRepository::new(pool);
        repo.set("theme", "dark").await.unwrap();
        let value = repo.get("theme").await.unwrap();
        assert_eq!(value, Some("dark".into()));
    }

    #[tokio::test]
    async fn test_get_missing() {
        let pool = create_test_db().await;
        let repo = SqlSettingsRepository::new(pool);
        let value = repo.get("nonexistent").await.unwrap();
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_get_all() {
        let pool = create_test_db().await;
        let repo = SqlSettingsRepository::new(pool);
        repo.set("key1", "val1").await.unwrap();
        repo.set("key2", "val2").await.unwrap();
        let all = repo.get_all().await.unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all.get("key1"), Some(&"val1".into()));
    }

    #[tokio::test]
    async fn test_overwrite() {
        let pool = create_test_db().await;
        let repo = SqlSettingsRepository::new(pool);
        repo.set("theme", "light").await.unwrap();
        repo.set("theme", "dark").await.unwrap();
        let value = repo.get("theme").await.unwrap();
        assert_eq!(value, Some("dark".into()));
    }
}
