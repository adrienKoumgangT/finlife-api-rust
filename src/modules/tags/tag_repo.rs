use anyhow::{Error, Result};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::tags::tag_model::Tag;
use crate::shared::state::AppState;


#[async_trait]
pub trait TagRepositoryInterface {
    
    async fn get(&self, tag_id: Uuid, user_id: Uuid) -> Result<Option<Tag>, Error>;
    
    async fn create(&self, tag: Tag, user_id: Uuid) -> Result<Tag, Error>;
    
    async fn delete(&self, tag_id: Uuid, user_id: Uuid) -> Result<(), Error>;
    
    async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<Tag>, Error>;
    
    async fn search_by_user(&self, user_id: Uuid, name: String) -> Result<Vec<Tag>, Error>;
    
}

#[derive(Clone)]
pub struct TagRepository {
    pool: MySqlPool,
}

impl From<&AppState> for TagRepository {
    fn from(app_state: &AppState) -> Self {
        Self {
            pool: app_state.mysql_pool.clone()
        }
    }
}

#[async_trait]
impl TagRepositoryInterface for TagRepository {
    async fn get(&self, tag_id: Uuid, user_id: Uuid) -> Result<Option<Tag>, Error> {
        let tag = sqlx::query_as!(
            Tag,
            r#"
            SELECT
                id AS "id: _",
                user_id AS "user_id: _",
                name,
                created_at
            FROM tags
            WHERE id = ? AND user_id = ?
            "#,
            tag_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;
        
        Ok(tag)
    }

    async fn create(&self, tag: Tag, user_id: Uuid) -> Result<Tag, Error> {
        let new_id = Uuid::new_v4();
        
        sqlx::query!(
            r#"
            INSERT INTO tags
                (id, user_id, name)
            VALUES
                (?, ?, ?)
            "#,
            new_id,
            tag.user_id,
            tag.name
        )
            .execute(&self.pool)
            .await?;
        
        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Tag not found after creation"))
    }

    async fn delete(&self, tag_id: Uuid, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!("DELETE FROM tags WHERE id = ? AND user_id = ?", tag_id, user_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<Tag>, Error> {
        let tags = sqlx::query_as!(
            Tag,
            r#"
            SELECT
                id AS "id: _",
                user_id AS "user_id: _",
                name,
                created_at
            FROM tags
            WHERE user_id = ?
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await?;
        
        Ok(tags)
    }

    async fn search_by_user(&self, user_id: Uuid, name: String) -> Result<Vec<Tag>, Error> {
        let tags = sqlx::query_as!(
            Tag,
            r#"
            SELECT
                id AS "id: _",
                user_id AS "user_id: _",
                name,
                created_at
            FROM tags
            WHERE user_id = ? AND name LIKE CONCAT("%", ?, "%")
            "#,
            user_id,
            name
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(tags)
    }
}
