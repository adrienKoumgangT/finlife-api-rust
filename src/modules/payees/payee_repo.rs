use anyhow::{Error, Result};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::payees::payee_model::Payee;
use crate::shared::state::AppState;

#[async_trait]
pub trait PayeeRepositoryInterface {

    async fn get(&self, payee_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Payee>, Error>;

    async fn create(&self, payee: Payee, user_id: Option<Uuid>) -> Result<Payee, Error>;

    async fn update(&self, payee_id: Uuid, name: String, user_id: Option<Uuid>) -> Result<Option<Payee>, Error>;

    async fn delete(&self, payee_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Payee>, Error>;

}

#[derive(Clone)]
pub struct PayeeRepository {
    pool: MySqlPool,
}

impl From<&AppState> for PayeeRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl PayeeRepositoryInterface for PayeeRepository {

    async fn get(&self, payee_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Payee>, Error> {
        let payee = sqlx::query_as!(
            Payee,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, created_at
            FROM payees
            WHERE id = ? AND user_id = ?
            "#,
            payee_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(payee)
    }

    async fn create(&self, payee: Payee, user_id: Option<Uuid>) -> Result<Payee, Error> {
        let new_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO payees
                (id, user_id, name)
            VALUES
                (?, ?, ?)
            "#,
            new_id,
            payee.user_id,
            payee.name
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Payee not found after creation"))
    }

    async fn update(&self, payee_id: Uuid, name: String, user_id: Option<Uuid>) -> Result<Option<Payee>, Error> {
        sqlx::query!(
            "UPDATE payees SET name = ? WHERE id = ? AND user_id = ?",
            name, payee_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(payee_id, user_id).await
    }

    async fn delete(&self, payee_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!("DELETE FROM payees WHERE id = ? AND user_id = ?", payee_id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Payee>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let payees = sqlx::query_as!(
            Payee,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, created_at
            FROM payees
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(payees)
    }
}
