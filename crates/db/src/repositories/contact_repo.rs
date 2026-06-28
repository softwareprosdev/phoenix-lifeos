use async_trait::async_trait;
use chrono::Utc;
use phoenix_core::entities::{Contact, ContactInteraction, InteractionType};
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

// ─── Contact ────────────────────────────────────────────────────────────────

#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn create(&self, contact: Contact) -> anyhow::Result<Contact>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Contact>>;
    async fn update(&self, contact: Contact) -> anyhow::Result<Contact>;
    async fn soft_delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn search(&self, query: &str) -> anyhow::Result<Vec<Contact>>;
}

fn map_contact(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<Contact> {
    let deleted_at: Option<String> = row.get("deleted_at");
    Ok(Contact {
        id: row.get::<String, _>("id").parse()?,
        name: row.get("name"),
        email: row.get("email"),
        phone: row.get("phone"),
        organization: row.get("organization"),
        notes: row.get("notes"),
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
        deleted_at: deleted_at.map(|s| s.parse()).transpose()?,
    })
}

#[derive(Debug, Clone)]
pub struct SqlContactRepository {
    pool: DbPool,
}

impl SqlContactRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContactRepository for SqlContactRepository {
    async fn create(&self, contact: Contact) -> anyhow::Result<Contact> {
        sqlx::query(
            "INSERT INTO contacts (id, name, email, phone, organization, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(contact.id.to_string())
        .bind(&contact.name)
        .bind(&contact.email)
        .bind(&contact.phone)
        .bind(&contact.organization)
        .bind(&contact.notes)
        .bind(contact.created_at.to_rfc3339())
        .bind(contact.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(contact.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created contact not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Contact>> {
        let row = sqlx::query("SELECT * FROM contacts WHERE id = ? AND deleted_at IS NULL")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_contact).transpose()
    }

    async fn update(&self, contact: Contact) -> anyhow::Result<Contact> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE contacts SET name = ?, email = ?, phone = ?, organization = ?, notes = ?, updated_at = ? WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(&contact.name)
        .bind(&contact.email)
        .bind(&contact.phone)
        .bind(&contact.organization)
        .bind(&contact.notes)
        .bind(&now)
        .bind(contact.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(contact.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated contact not found"))
    }

    async fn soft_delete(&self, id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE contacts SET deleted_at = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&now)
        .bind(&now)
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<Contact>> {
        if query.trim().is_empty() {
            let rows = sqlx::query(
                "SELECT * FROM contacts WHERE deleted_at IS NULL ORDER BY name",
            )
            .fetch_all(&self.pool)
            .await?;
            return rows.iter().map(map_contact).collect();
        }
        let pattern = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let rows = sqlx::query(
            "SELECT * FROM contacts WHERE deleted_at IS NULL AND (name LIKE ? ESCAPE '\\' OR email LIKE ? ESCAPE '\\' OR organization LIKE ? ESCAPE '\\') ORDER BY name",
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_contact).collect()
    }
}

// ─── ContactInteraction ─────────────────────────────────────────────────────

#[async_trait]
pub trait ContactInteractionRepository: Send + Sync {
    async fn create(&self, interaction: ContactInteraction) -> anyhow::Result<ContactInteraction>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<ContactInteraction>>;
    async fn update(&self, interaction: ContactInteraction) -> anyhow::Result<ContactInteraction>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_by_contact(&self, contact_id: Uuid) -> anyhow::Result<Vec<ContactInteraction>>;
}

fn map_contact_interaction(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<ContactInteraction> {
    let it_str: String = row.get("interaction_type");
    let interaction_type = match it_str.as_str() {
        "call" => InteractionType::Call,
        "email" => InteractionType::Email,
        "meeting" => InteractionType::Meeting,
        "note" => InteractionType::Note,
        "other" => InteractionType::Other,
        other => anyhow::bail!("invalid interaction_type: {other}"),
    };
    Ok(ContactInteraction {
        id: row.get::<String, _>("id").parse()?,
        contact_id: row.get::<String, _>("contact_id").parse()?,
        interaction_type,
        subject: row.get("subject"),
        notes: row.get("notes"),
        date: row.get::<String, _>("date").parse()?,
        created_at: row.get::<String, _>("created_at").parse()?,
    })
}

fn interaction_type_str(t: &InteractionType) -> &'static str {
    match t {
        InteractionType::Call => "call",
        InteractionType::Email => "email",
        InteractionType::Meeting => "meeting",
        InteractionType::Note => "note",
        InteractionType::Other => "other",
    }
}

#[derive(Debug, Clone)]
pub struct SqlContactInteractionRepository {
    pool: DbPool,
}

impl SqlContactInteractionRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContactInteractionRepository for SqlContactInteractionRepository {
    async fn create(
        &self,
        interaction: ContactInteraction,
    ) -> anyhow::Result<ContactInteraction> {
        sqlx::query(
            "INSERT INTO contact_interactions (id, contact_id, interaction_type, subject, notes, date, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(interaction.id.to_string())
        .bind(interaction.contact_id.to_string())
        .bind(interaction_type_str(&interaction.interaction_type))
        .bind(&interaction.subject)
        .bind(&interaction.notes)
        .bind(interaction.date.to_rfc3339())
        .bind(interaction.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(interaction.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created interaction not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<ContactInteraction>> {
        let row = sqlx::query("SELECT * FROM contact_interactions WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_contact_interaction).transpose()
    }

    async fn update(
        &self,
        interaction: ContactInteraction,
    ) -> anyhow::Result<ContactInteraction> {
        sqlx::query(
            "UPDATE contact_interactions SET contact_id = ?, interaction_type = ?, subject = ?, notes = ?, date = ? WHERE id = ?",
        )
        .bind(interaction.contact_id.to_string())
        .bind(interaction_type_str(&interaction.interaction_type))
        .bind(&interaction.subject)
        .bind(&interaction.notes)
        .bind(interaction.date.to_rfc3339())
        .bind(interaction.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(interaction.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated interaction not found"))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM contact_interactions WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_contact(&self, contact_id: Uuid) -> anyhow::Result<Vec<ContactInteraction>> {
        let rows = sqlx::query(
            "SELECT * FROM contact_interactions WHERE contact_id = ? ORDER BY date DESC",
        )
        .bind(contact_id.to_string())
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(map_contact_interaction).collect()
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
    async fn test_contact_crud() {
        let pool = create_test_db().await;
        let repo = SqlContactRepository::new(pool);
        let contact = Contact {
            id: Uuid::new_v4(),
            name: "John Doe".into(),
            email: Some("john@example.com".into()),
            phone: None,
            organization: Some("Acme Corp".into()),
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };
        let created = repo.create(contact).await.unwrap();
        assert_eq!(created.name, "John Doe");
        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_some());
    }

    #[tokio::test]
    async fn test_contact_search() {
        let pool = create_test_db().await;
        let repo = SqlContactRepository::new(pool);
        let contact = Contact {
            id: Uuid::new_v4(),
            name: "Jane Smith".into(),
            email: Some("jane@example.com".into()),
            phone: None,
            organization: None,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };
        repo.create(contact).await.unwrap();
        let results = repo.search("Jane").await.unwrap();
        assert_eq!(results.len(), 1);
        let no_results = repo.search("NonExistent").await.unwrap();
        assert!(no_results.is_empty());
    }
}
