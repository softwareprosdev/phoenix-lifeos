use chrono::Utc;
use phoenix_core::entities::{Document, DocumentSource};
use phoenix_db::repositories::document_repo::{
    DocumentRepository as _, SqlDocumentRepository,
};
use phoenix_db::DbPool;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateDocumentInput {
    pub name: String,
    pub file_type: String,
    pub mime_type: Option<String>,
    pub encrypted_path: String,
    pub source: DocumentSource,
    pub tags: Vec<String>,
    pub metadata: String,
}

#[derive(Debug)]
pub struct UpdateDocumentInput {
    pub id: Uuid,
    pub name: String,
    pub file_type: String,
    pub mime_type: Option<String>,
    pub encrypted_path: String,
    pub source: DocumentSource,
    pub tags: Vec<String>,
    pub metadata: String,
}

pub struct DocumentService {
    repo: SqlDocumentRepository,
}

impl DocumentService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            repo: SqlDocumentRepository::new(pool),
        }
    }

    #[instrument(skip(self))]
    pub async fn create(&self, input: CreateDocumentInput) -> anyhow::Result<Document> {
        let now = Utc::now();
        let doc = Document {
            id: Uuid::new_v4(),
            name: input.name,
            file_type: input.file_type,
            mime_type: input.mime_type,
            encrypted_path: input.encrypted_path,
            ocr_text_encrypted: None,
            extracted_metadata: input.metadata,
            tags: input.tags,
            source: input.source,
            created_at: now,
            updated_at: now,
        };
        self.repo.create(doc).await
    }

    #[instrument(skip(self))]
    pub async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Document>> {
        self.repo.get_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn update(&self, input: UpdateDocumentInput) -> anyhow::Result<Document> {
        let now = Utc::now();
        let doc = Document {
            id: input.id,
            name: input.name,
            file_type: input.file_type,
            mime_type: input.mime_type,
            encrypted_path: input.encrypted_path,
            ocr_text_encrypted: None,
            extracted_metadata: input.metadata,
            tags: input.tags,
            source: input.source,
            created_at: now,
            updated_at: now,
        };
        self.repo.update(doc).await
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        self.repo.delete(id).await
    }

    #[instrument(skip(self))]
    pub async fn list_all(&self) -> anyhow::Result<Vec<Document>> {
        self.repo.list_all().await
    }

    #[instrument(skip(self))]
    pub async fn search(&self, query: &str) -> anyhow::Result<Vec<Document>> {
        self.repo.search(query).await
    }
}
