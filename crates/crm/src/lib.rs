use chrono::{DateTime, Utc};
use phoenix_core::entities::{Contact, ContactInteraction, InteractionType};
use phoenix_db::repositories::contact_repo::{
    ContactInteractionRepository as _, ContactRepository as _, SqlContactInteractionRepository,
    SqlContactRepository,
};
use phoenix_db::DbPool;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateContactInput {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub organization: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct UpdateContactInput {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub organization: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct CreateInteractionInput {
    pub contact_id: Uuid,
    pub interaction_type: InteractionType,
    pub subject: Option<String>,
    pub notes: Option<String>,
    pub date: DateTime<Utc>,
}

#[derive(Debug)]
pub struct UpdateInteractionInput {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub interaction_type: InteractionType,
    pub subject: Option<String>,
    pub notes: Option<String>,
    pub date: DateTime<Utc>,
}

pub struct CrmService {
    contact_repo: SqlContactRepository,
    interaction_repo: SqlContactInteractionRepository,
}

impl CrmService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            contact_repo: SqlContactRepository::new(pool.clone()),
            interaction_repo: SqlContactInteractionRepository::new(pool),
        }
    }

    // ─── Contacts ────────────────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn create_contact(&self, input: CreateContactInput) -> anyhow::Result<Contact> {
        let now = Utc::now();
        let contact = Contact {
            id: Uuid::new_v4(),
            name: input.name,
            email: input.email,
            phone: input.phone,
            organization: input.organization,
            notes: input.notes,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        self.contact_repo.create(contact).await
    }

    #[instrument(skip(self))]
    pub async fn get_contact(&self, id: Uuid) -> anyhow::Result<Option<Contact>> {
        self.contact_repo.get_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn update_contact(&self, input: UpdateContactInput) -> anyhow::Result<Contact> {
        let now = Utc::now();
        let contact = Contact {
            id: input.id,
            name: input.name,
            email: input.email,
            phone: input.phone,
            organization: input.organization,
            notes: input.notes,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        self.contact_repo.update(contact).await
    }

    #[instrument(skip(self))]
    pub async fn soft_delete_contact(&self, id: Uuid) -> anyhow::Result<()> {
        self.contact_repo.soft_delete(id).await
    }

    #[instrument(skip(self))]
    pub async fn search_contacts(&self, query: &str) -> anyhow::Result<Vec<Contact>> {
        self.contact_repo.search(query).await
    }

    // ─── Interactions ────────────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn create_interaction(
        &self,
        input: CreateInteractionInput,
    ) -> anyhow::Result<ContactInteraction> {
        let interaction = ContactInteraction {
            id: Uuid::new_v4(),
            contact_id: input.contact_id,
            interaction_type: input.interaction_type,
            subject: input.subject,
            notes: input.notes,
            date: input.date,
            created_at: Utc::now(),
        };
        self.interaction_repo.create(interaction).await
    }

    #[instrument(skip(self))]
    pub async fn get_interaction(&self, id: Uuid) -> anyhow::Result<Option<ContactInteraction>> {
        self.interaction_repo.get_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn update_interaction(
        &self,
        input: UpdateInteractionInput,
    ) -> anyhow::Result<ContactInteraction> {
        let interaction = ContactInteraction {
            id: input.id,
            contact_id: input.contact_id,
            interaction_type: input.interaction_type,
            subject: input.subject,
            notes: input.notes,
            date: input.date,
            created_at: Utc::now(),
        };
        self.interaction_repo.update(interaction).await
    }

    #[instrument(skip(self))]
    pub async fn delete_interaction(&self, id: Uuid) -> anyhow::Result<()> {
        self.interaction_repo.delete(id).await
    }

    #[instrument(skip(self))]
    pub async fn list_interactions(&self, contact_id: Uuid) -> anyhow::Result<Vec<ContactInteraction>> {
        self.interaction_repo.list_by_contact(contact_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY, name TEXT NOT NULL,
                email TEXT, phone TEXT, organization TEXT, notes TEXT,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS contact_interactions (
                id TEXT PRIMARY KEY, contact_id TEXT NOT NULL,
                interaction_type TEXT NOT NULL, subject TEXT, notes TEXT,
                date TEXT NOT NULL, created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_contact() {
        let pool = create_test_db().await;
        let svc = CrmService::new(pool);
        let contact = svc
            .create_contact(CreateContactInput {
                name: "John Doe".into(),
                email: Some("john@example.com".into()),
                phone: None,
                organization: Some("Acme Corp".into()),
                notes: None,
            })
            .await
            .unwrap();
        assert_eq!(contact.name, "John Doe");
        assert_eq!(contact.email.as_deref(), Some("john@example.com"));
    }

    #[tokio::test]
    async fn test_search_contacts() {
        let pool = create_test_db().await;
        let svc = CrmService::new(pool);
        svc.create_contact(CreateContactInput {
            name: "Jane Smith".into(),
            email: Some("jane@example.com".into()),
            phone: None,
            organization: None,
            notes: None,
        })
        .await
        .unwrap();

        let results = svc.search_contacts("Jane").await.unwrap();
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_create_interaction() {
        let pool = create_test_db().await;
        let svc = CrmService::new(pool);
        let contact = svc
            .create_contact(CreateContactInput {
                name: "Alice".into(),
                email: None,
                phone: None,
                organization: None,
                notes: None,
            })
            .await
            .unwrap();

        let interaction = svc
            .create_interaction(CreateInteractionInput {
                contact_id: contact.id,
                interaction_type: InteractionType::Call,
                subject: Some("Follow up".into()),
                notes: Some("Discussed proposal".into()),
                date: Utc::now(),
            })
            .await
            .unwrap();
        assert_eq!(interaction.interaction_type, InteractionType::Call);

        let interactions = svc.list_interactions(contact.id).await.unwrap();
        assert_eq!(interactions.len(), 1);
    }

    #[tokio::test]
    async fn test_soft_delete_contact() {
        let pool = create_test_db().await;
        let svc = CrmService::new(pool);
        let contact = svc
            .create_contact(CreateContactInput {
                name: "Bob".into(),
                email: None,
                phone: None,
                organization: None,
                notes: None,
            })
            .await
            .unwrap();

        svc.soft_delete_contact(contact.id).await.unwrap();
        let fetched = svc.get_contact(contact.id).await.unwrap();
        assert!(fetched.is_none());
    }
}
