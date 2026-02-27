use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::reviews::sessions::review_session_model::{ReviewSession, ReviewStatus};
use crate::shared::state::AppState;


#[async_trait]
pub trait ReviewSessionRepositoryInterface {
    
    async fn get(&self, review_session_id: Uuid, user_id: Uuid) -> Result<Option<ReviewSession>, Error>;
    
    async fn create(&self, review: ReviewSession, user_id: Uuid) -> Result<ReviewSession, Error>;
    
    #[allow(clippy::too_many_arguments)]
    async fn update(&self, review_session_id: Uuid, period_start: NaiveDate, period_end: NaiveDate, status: ReviewStatus, notes: Option<String>, actions: Option<serde_json::Value>, decisions: Option<serde_json::Value>, user_id: Uuid) -> Result<Option<ReviewSession>, Error>;
    
    async fn delete(&self, review_session_id: Uuid, user_id: Uuid) -> Result<(), Error>;
    
    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<ReviewSession>, Error>;
    
}

#[derive(Clone)]
pub struct ReviewSessionRepository {
    pool: MySqlPool,
}

impl From<&AppState> for ReviewSessionRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl ReviewSessionRepositoryInterface for ReviewSessionRepository {

    async fn get(&self, review_session_id: Uuid, user_id: Uuid) -> Result<Option<ReviewSession>, Error> {
        let review = sqlx::query_as!(
            ReviewSession,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                review_type AS "review_type: String",
                period_start, period_end,
                status AS "status: String",
                notes, actions, decisions,
                created_at, updated_at
            FROM review_sessions
            WHERE id = ? AND user_id = ?
            "#,
            review_session_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(review)
    }

    async fn create(&self, review: ReviewSession, user_id: Uuid) -> Result<ReviewSession, Error> {
        let new_id = Uuid::new_v4();
        let type_str = review.review_type.as_str();
        let status_str = review.status.as_str();

        sqlx::query!(
            r#"
            INSERT INTO review_sessions
                (id, user_id, review_type, period_start, period_end, status, notes, actions, decisions)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id, review.user_id, type_str, review.period_start, review.period_end,
            status_str, review.notes, review.actions, review.decisions
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Review session not found after creation"))
    }

    async fn update(&self, review_session_id: Uuid, period_start: NaiveDate, period_end: NaiveDate, status: ReviewStatus, notes: Option<String>, actions: Option<serde_json::Value>, decisions: Option<serde_json::Value>, user_id: Uuid) -> Result<Option<ReviewSession>, Error> {
        let status_str = status.as_str();

        sqlx::query!(
            r#"
            UPDATE review_sessions
            SET period_start = ?, period_end = ?, status = ?, notes = ?, actions = ?, decisions = ?
            WHERE id = ? AND user_id = ?
            "#,
            period_start, period_end, status_str, notes, actions, decisions, review_session_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(review_session_id, user_id).await
    }

    async fn delete(&self, review_session_id: Uuid, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!("DELETE FROM review_sessions WHERE id = ? AND user_id = ?", review_session_id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<ReviewSession>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let reviews = sqlx::query_as!(
            ReviewSession,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                review_type AS "review_type: String",
                period_start, period_end,
                status AS "status: String",
                notes, actions, decisions,
                created_at, updated_at
            FROM review_sessions
            WHERE user_id = ?
            ORDER BY period_start DESC, created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(reviews)
    }
}
