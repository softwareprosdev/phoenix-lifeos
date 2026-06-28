use async_trait::async_trait;
use phoenix_core::entities::Event;
use sqlx::Row;

use crate::DbPool;

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn publish(&self, event: Event) -> anyhow::Result<Event>;
    async fn list_by_type(&self, event_type: &str) -> anyhow::Result<Vec<Event>>;
    async fn list_by_entity(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> anyhow::Result<Vec<Event>>;
    async fn list_recent(&self, limit: i64) -> anyhow::Result<Vec<Event>>;
}

fn map_event(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<Event> {
    Ok(Event {
        id: row.get::<String, _>("id").parse()?,
        event_type: row.get("event_type"),
        entity_type: row.get("entity_type"),
        entity_id: row.get("entity_id"),
        metadata: row.get("metadata"),
        created_at: row.get::<String, _>("created_at").parse()?,
    })
}

#[derive(Debug, Clone)]
pub struct SqlEventRepository {
    pool: DbPool,
}

impl SqlEventRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventRepository for SqlEventRepository {
    async fn publish(&self, event: Event) -> anyhow::Result<Event> {
        sqlx::query(
            "INSERT INTO events_log (id, event_type, entity_type, entity_id, metadata, created_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(event.id.to_string())
        .bind(&event.event_type)
        .bind(&event.entity_type)
        .bind(&event.entity_id)
        .bind(&event.metadata)
        .bind(event.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(event)
    }

    async fn list_by_type(&self, event_type: &str) -> anyhow::Result<Vec<Event>> {
        let rows = sqlx::query(
            "SELECT * FROM events_log WHERE event_type = ? ORDER BY created_at DESC",
        )
        .bind(event_type)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_event).collect()
    }

    async fn list_by_entity(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> anyhow::Result<Vec<Event>> {
        let rows = sqlx::query(
            "SELECT * FROM events_log WHERE entity_type = ? AND entity_id = ? ORDER BY created_at DESC",
        )
        .bind(entity_type)
        .bind(entity_id)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_event).collect()
    }

    async fn list_recent(&self, limit: i64) -> anyhow::Result<Vec<Event>> {
        let rows = sqlx::query("SELECT * FROM events_log ORDER BY created_at DESC LIMIT ?")
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_event).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS events_log (
                id TEXT PRIMARY KEY, event_type TEXT NOT NULL,
                entity_type TEXT, entity_id TEXT, metadata TEXT,
                created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_publish_and_list() {
        let pool = create_test_db().await;
        let repo = SqlEventRepository::new(pool);
        let event = Event {
            id: Uuid::new_v4(),
            event_type: "account.created".into(),
            entity_type: Some("account".into()),
            entity_id: Some(Uuid::new_v4().to_string()),
            metadata: Some(r#"{"name":"Test"}"#.into()),
            created_at: Utc::now(),
        };
        repo.publish(event).await.unwrap();
        let events = repo.list_by_type("account.created").await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "account.created");
    }

    #[tokio::test]
    async fn test_list_recent() {
        let pool = create_test_db().await;
        let repo = SqlEventRepository::new(pool);
        for _ in 0..5 {
            let event = Event {
                id: Uuid::new_v4(),
                event_type: "test.event".into(),
                entity_type: None,
                entity_id: None,
                metadata: None,
                created_at: Utc::now(),
            };
            repo.publish(event).await.unwrap();
        }
        let recent = repo.list_recent(3).await.unwrap();
        assert_eq!(recent.len(), 3);
    }
}
