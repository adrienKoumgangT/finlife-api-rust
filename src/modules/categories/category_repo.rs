use anyhow::{Error, Result};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::categories::category_model::{Category, CategoryKind};
use crate::shared::state::AppState;

#[async_trait]
pub trait CategoryRepositoryInterface {

    async fn get(&self, category_id: Uuid, user_id: Uuid) -> Result<Option<Category>, Error>;

    async fn create(&self, category: Category, user_id: Uuid) -> Result<Category, Error>;

    async fn update(&self, category_id: Uuid, name: String, parent_id: Option<Uuid>, sort_order: i32, user_id: Uuid) -> Result<Option<Category>, Error>;

    async fn archived(&self, category_id: Uuid, archived: bool, user_id: Uuid) -> Result<Option<Category>, Error>;

    async fn delete(&self, category_id: Uuid, user_id: Uuid) -> Result<(), Error>;

    async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<Category>, Error>;

}

#[derive(Clone)]
pub struct CategoryRepository {
    pool: MySqlPool,
}

impl From<&AppState> for CategoryRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl CategoryRepositoryInterface for CategoryRepository {

    async fn get(&self, category_id: Uuid, user_id: Uuid) -> Result<Option<Category>, Error> {
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, kind AS "kind: String", parent_id AS "parent_id: _", sort_order,
                archived AS "archived: bool",
                created_at, updated_at
            FROM categories
            WHERE id = ? AND user_id = ?
            "#,
            category_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(category)
    }

    async fn create(&self, category: Category, user_id: Uuid) -> Result<Category, Error> {
        let new_id = Uuid::new_v4();
        let kind_str = category.kind.as_str();

        sqlx::query!(
            r#"
            INSERT INTO categories
                (id, user_id, name, kind, parent_id, sort_order, archived)
            VALUES
                (?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id,
            category.user_id,
            category.name,
            kind_str,
            category.parent_id,
            category.sort_order,
            category.archived
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Category not found after creation"))
    }

    async fn update(&self, category_id: Uuid, name: String, parent_id: Option<Uuid>, sort_order: i32, user_id: Uuid) -> Result<Option<Category>, Error> {
        sqlx::query!(
            "UPDATE categories SET name = ?, parent_id = ?, sort_order = ? WHERE id = ? AND user_id = ?",
            name, parent_id, sort_order, category_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(category_id, user_id).await
    }

    async fn archived(&self, category_id: Uuid, archived: bool, user_id: Uuid) -> Result<Option<Category>, Error> {
        sqlx::query!(
            "UPDATE categories SET archived = ? WHERE id = ? AND user_id = ?",
            archived,
            category_id,
            user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(category_id, user_id).await
    }

    async fn delete(&self, category_id: Uuid, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!("DELETE FROM categories WHERE id = ? AND user_id = ?", category_id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<Category>, Error> {
        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, kind AS "kind: String", parent_id AS "parent_id: _", sort_order,
                archived AS "archived: bool",
                created_at, updated_at
            FROM categories
            WHERE user_id = ?
            ORDER BY sort_order ASC, created_at DESC
            "#,
            user_id,
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(categories)
    }
}
