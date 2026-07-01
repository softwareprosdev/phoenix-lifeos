use async_trait::async_trait;
use phoenix_core::entities::{Document, DocumentSource};
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn create(&self, doc: Document) -> anyhow::Result<Document>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Document>>;
    async fn update(&self, doc: Document) -> anyhow::Result<Document>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_all(&self) -> anyhow::Result<Vec<Document>>;
    async fn search(&self, query: &str) -> anyhow::Result<Vec<Document>>;
}

fn map_document(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<Document> {
    let source_str: String = row.get("source");
    let source = match source_str.as_str() {
        "upload" => DocumentSource::Upload,
        "email" => DocumentSource::Email,
        "scan" => DocumentSource::Scan,
        other => anyhow::bail!("invalid document source: {other}"),
    };
    Ok(Document {
        id: row.get::<String, _>("id").parse()?,
        name: row.get("name"),
        file_type: row.get("file_type"),
        mime_type: row.get("mime_type"),
        encrypted_path: row.get("encrypted_path"),
        ocr_text_encrypted: row.get("ocr_text_encrypted"),
        extracted_metadata: row.get("extracted_metadata"),
        tags: serde_json::from_str(&row.get::<String, _>("tags"))?,
        source,
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn document_source_str(s: &DocumentSource) -> &'static str {
    match s {
        DocumentSource::Upload => "upload",
        DocumentSource::Email => "email",
        DocumentSource::Scan => "scan",
    }
}

#[derive(Debug, Clone)]
pub struct SqlDocumentRepository {
    pool: DbPool,
}

impl SqlDocumentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DocumentRepository for SqlDocumentRepository {
    async fn create(&self, doc: Document) -> anyhow::Result<Document> {
        sqlx::query(
            "INSERT INTO documents (id, name, file_type, mime_type, encrypted_path, ocr_text_encrypted, extracted_metadata, tags, source, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(doc.id.to_string())
        .bind(&doc.name)
        .bind(&doc.file_type)
        .bind(&doc.mime_type)
        .bind(&doc.encrypted_path)
        .bind(&doc.ocr_text_encrypted)
        .bind(&doc.extracted_metadata)
        .bind(serde_json::to_string(&doc.tags)?)
        .bind(document_source_str(&doc.source))
        .bind(doc.created_at.to_rfc3339())
        .bind(doc.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(doc.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created document not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Document>> {
        let row = sqlx::query("SELECT * FROM documents WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_document).transpose()
    }

    async fn update(&self, doc: Document) -> anyhow::Result<Document> {
        sqlx::query(
            "UPDATE documents SET name = ?, file_type = ?, mime_type = ?, encrypted_path = ?, ocr_text_encrypted = ?, extracted_metadata = ?, tags = ?, source = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&doc.name)
        .bind(&doc.file_type)
        .bind(&doc.mime_type)
        .bind(&doc.encrypted_path)
        .bind(&doc.ocr_text_encrypted)
        .bind(&doc.extracted_metadata)
        .bind(serde_json::to_string(&doc.tags)?)
        .bind(document_source_str(&doc.source))
        .bind(doc.updated_at.to_rfc3339())
        .bind(doc.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(doc.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated document not found"))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM documents WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_all(&self) -> anyhow::Result<Vec<Document>> {
        let rows = sqlx::query("SELECT * FROM documents ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_document).collect()
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<Document>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }
        let pattern = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let rows = sqlx::query(
            "SELECT * FROM documents WHERE name LIKE ? ESCAPE '\\' OR tags LIKE ? ESCAPE '\\' ORDER BY created_at DESC",
        )
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_document).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY, name TEXT NOT NULL,
                file_type TEXT NOT NULL, mime_type TEXT,
                encrypted_path TEXT NOT NULL, ocr_text_encrypted TEXT,
                extracted_metadata TEXT NOT NULL DEFAULT '{}',
                tags TEXT NOT NULL DEFAULT '[]',
                source TEXT NOT NULL DEFAULT 'upload',
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    fn sample_doc() -> Document {
        Document {
            id: Uuid::new_v4(),
            name: "test.pdf".into(),
            file_type: "pdf".into(),
            mime_type: Some("application/pdf".into()),
            encrypted_path: "encrypted/path".into(),
            ocr_text_encrypted: None,
            extracted_metadata: "{}".into(),
            tags: vec!["important".into()],
            source: DocumentSource::Upload,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_document_crud() {
        let pool = create_test_db().await;
        let repo = SqlDocumentRepository::new(pool);
        let doc = sample_doc();
        let created = repo.create(doc).await.unwrap();
        assert_eq!(created.name, "test.pdf");

        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().file_type, "pdf");

        let all = repo.list_all().await.unwrap();
        assert_eq!(all.len(), 1);

        repo.delete(created.id).await.unwrap();
        let after = repo.list_all().await.unwrap();
        assert!(after.is_empty());
    }

    #[tokio::test]
    async fn test_document_search() {
        let pool = create_test_db().await;
        let repo = SqlDocumentRepository::new(pool);
        repo.create(sample_doc()).await.unwrap();

        let results = repo.search("test").await.unwrap();
        assert_eq!(results.len(), 1);

        let no_results = repo.search("nonexistent").await.unwrap();
        assert!(no_results.is_empty());
    }
}
