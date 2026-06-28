use async_trait::async_trait;
use phoenix_core::entities::{Category, CategoryType};
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn create(&self, category: Category) -> anyhow::Result<Category>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>>;
    async fn update(&self, category: Category) -> anyhow::Result<Category>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_by_type(&self, category_type: CategoryType) -> anyhow::Result<Vec<Category>>;
    async fn list_tree(&self) -> anyhow::Result<Vec<Category>>;
}

#[derive(Debug, Clone)]
pub struct SqlCategoryRepository {
    pool: DbPool,
}

impl SqlCategoryRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn map_category(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<Category> {
    let ct_str: String = row.get("type");
    let category_type = match ct_str.as_str() {
        "income" => CategoryType::Income,
        "expense" => CategoryType::Expense,
        other => anyhow::bail!("invalid category type: {other}"),
    };
    Ok(Category {
        id: row.get::<String, _>("id").parse()?,
        name: row.get("name"),
        category_type,
        parent_id: row
            .get::<Option<String>, _>("parent_id")
            .map(|s| s.parse())
            .transpose()?,
        created_at: row.get::<String, _>("created_at").parse()?,
    })
}

fn category_type_str(t: &CategoryType) -> &'static str {
    match t {
        CategoryType::Income => "income",
        CategoryType::Expense => "expense",
    }
}

#[async_trait]
impl CategoryRepository for SqlCategoryRepository {
    async fn create(&self, category: Category) -> anyhow::Result<Category> {
        sqlx::query(
            "INSERT INTO categories (id, name, type, parent_id, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(category.id.to_string())
        .bind(&category.name)
        .bind(category_type_str(&category.category_type))
        .bind(category.parent_id.map(|p| p.to_string()))
        .bind(category.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_by_id(category.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created category not found"))
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>> {
        let row = sqlx::query("SELECT id, name, type, parent_id, created_at FROM categories WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_category).transpose()
    }

    async fn update(&self, category: Category) -> anyhow::Result<Category> {
        sqlx::query(
            "UPDATE categories SET name = ?, type = ?, parent_id = ? WHERE id = ?",
        )
        .bind(&category.name)
        .bind(category_type_str(&category.category_type))
        .bind(category.parent_id.map(|p| p.to_string()))
        .bind(category.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_by_id(category.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated category not found"))
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_type(&self, category_type: CategoryType) -> anyhow::Result<Vec<Category>> {
        let rows = sqlx::query("SELECT id, name, type, parent_id, created_at FROM categories WHERE type = ? ORDER BY name")
            .bind(category_type_str(&category_type))
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_category).collect()
    }

    async fn list_tree(&self) -> anyhow::Result<Vec<Category>> {
        let rows = sqlx::query("SELECT id, name, type, parent_id, created_at FROM categories ORDER BY type, name")
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_category).collect()
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
            "CREATE TABLE IF NOT EXISTS categories (
                id TEXT PRIMARY KEY, name TEXT NOT NULL,
                type TEXT NOT NULL, parent_id TEXT,
                created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    fn test_cat(category_type: CategoryType) -> Category {
        Category {
            id: Uuid::new_v4(),
            name: "Test Category".into(),
            category_type,
            parent_id: None,
            created_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let pool = create_test_db().await;
        let repo = SqlCategoryRepository::new(pool);
        let cat = test_cat(CategoryType::Expense);
        let created = repo.create(cat).await.unwrap();
        assert_eq!(created.name, "Test Category");
        let fetched = repo.get_by_id(created.id).await.unwrap();
        assert!(fetched.is_some());
    }

    #[tokio::test]
    async fn test_list_by_type() {
        let pool = create_test_db().await;
        let repo = SqlCategoryRepository::new(pool);
        let exp = test_cat(CategoryType::Expense);
        repo.create(exp).await.unwrap();
        let inc = Category {
            id: Uuid::new_v4(),
            name: "Salary".into(),
            category_type: CategoryType::Income,
            parent_id: None,
            created_at: Utc::now(),
        };
        repo.create(inc).await.unwrap();
        let expenses = repo.list_by_type(CategoryType::Expense).await.unwrap();
        assert_eq!(expenses.len(), 1);
        let incomes = repo.list_by_type(CategoryType::Income).await.unwrap();
        assert_eq!(incomes.len(), 1);
    }
}
