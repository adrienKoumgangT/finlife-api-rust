use anyhow::{Error, Result};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::people::people_model::People;
use crate::shared::state::AppState;

#[async_trait]
pub trait PeopleRepositoryInterface {

    async fn get(&self, people_id: Uuid, user_id: Option<Uuid>) -> Result<Option<People>, Error>;

    async fn create(&self, people: People, user_id: Option<Uuid>) -> Result<People, Error>;

    async fn update_image(&self, people_id: Uuid, image: Option<Uuid>, user_id: Option<Uuid>) -> Result<Option<People>, Error>;

    async fn update(&self, people_id: Uuid, name: String, email: Option<String>, phone: Option<String>, note: Option<String>, user_id: Option<Uuid>) -> Result<Option<People>, Error>;

    async fn archived(&self, people_id: Uuid, archived: bool, user_id: Option<Uuid>) -> Result<Option<People>, Error>;

    async fn delete(&self, people_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<People>, Error>;

    async fn search_by_user(&self, user_id: Uuid, query: String) -> Result<Vec<People>, Error>;

}


#[derive(Clone)]
pub struct PeopleRepository {
    pool: MySqlPool,
}

impl From<&AppState> for PeopleRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl PeopleRepositoryInterface for PeopleRepository {

    async fn get(&self, people_id: Uuid, user_id: Option<Uuid>) -> Result<Option<People>, Error> {
        let people = sqlx::query_as!(
            People,
            r#"
            SELECT
                id AS "id: _",
                user_id AS "user_id: _",
                name, email, phone, image AS "image: _", note,
                archived AS "archived: bool",
                created_at, updated_at
            FROM people
            WHERE id = ? AND user_id = ?
            "#,
            people_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(people)
    }

    async fn create(&self, people: People, user_id: Option<Uuid>) -> Result<People, Error> {
        let new_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO people
                (id, user_id, name, email, phone, image, note, archived)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id,
            people.user_id,
            people.name,
            people.email,
            people.phone,
            people.image,
            people.note,
            people.archived
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Person not found after creation"))
    }

    async fn update_image(&self, people_id: Uuid, image: Option<Uuid>, user_id: Option<Uuid>) -> Result<Option<People>, Error> {
        sqlx::query!(
            "UPDATE people SET image = ? WHERE id = ? AND user_id = ?",
            image,
            people_id,
            user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(people_id, user_id).await
    }

    async fn update(&self, people_id: Uuid, name: String, email: Option<String>, phone: Option<String>, note: Option<String>, user_id: Option<Uuid>) -> Result<Option<People>, Error> {
        sqlx::query!(
            "UPDATE people SET name = ?, email = ?, phone = ?, note = ? WHERE id = ? AND user_id = ?",
            name, email, phone, note, people_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(people_id, user_id).await
    }

    async fn archived(&self, people_id: Uuid, archived: bool, user_id: Option<Uuid>) -> Result<Option<People>, Error> {
        sqlx::query!(
            "UPDATE people SET archived = ? WHERE id = ? AND user_id = ?",
            archived,
            people_id,
            user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(people_id, user_id).await
    }

    async fn delete(&self, people_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!("DELETE FROM people WHERE id = ? AND user_id = ?", people_id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<People>, Error> {
        let people = sqlx::query_as!(
            People,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, email, phone, image AS "image: _", note,
                archived AS "archived: bool",
                created_at, updated_at
            FROM people
            WHERE user_id = ?
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(people)
    }

    async fn search_by_user(&self, user_id: Uuid, query: String) -> Result<Vec<People>, Error> {
        let search_term = format!("%{}%", query);

        let people = sqlx::query_as!(
            People,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, email, phone, image AS "image: _", note,
                archived AS "archived: bool",
                created_at, updated_at
            FROM people
            WHERE user_id = ? AND (name LIKE ? OR email LIKE ?)
            "#,
            user_id,
            search_term,
            search_term
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(people)
    }
}
