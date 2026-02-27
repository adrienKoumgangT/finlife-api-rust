use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::goals::goal_model::{Goal, GoalType};
use crate::shared::state::AppState;

#[async_trait]
pub trait GoalRepositoryInterface {

    async fn get(&self, goal_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Goal>, Error>;

    async fn create(&self, goal: Goal, user_id: Option<Uuid>) -> Result<Goal, Error>;

    async fn update(&self, goal_id: Uuid, name: String, goal_type: GoalType, target_base_minor: i64, target_date: Option<NaiveDate>, priority: i32, linked_account_id: Option<Uuid>, user_id: Option<Uuid>) -> Result<Option<Goal>, Error>;

    async fn delete(&self, goal_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Goal>, Error>;

}

#[derive(Clone)]
pub struct GoalRepository {
    pool: MySqlPool,
}

impl From<&AppState> for GoalRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl GoalRepositoryInterface for GoalRepository {

    async fn get(&self, goal_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Goal>, Error> {
        let goal = sqlx::query_as!(
            Goal,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, goal_type AS "goal_type: String",
                target_base_minor, target_date, priority,
                linked_account_id AS "linked_account_id: _",
                created_at, updated_at
            FROM goals
            WHERE id = ? AND user_id = ?
            "#,
            goal_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(goal)
    }

    async fn create(&self, goal: Goal, user_id: Option<Uuid>) -> Result<Goal, Error> {
        let new_id = Uuid::new_v4();
        let type_str = goal.goal_type.as_str();

        sqlx::query!(
            r#"
            INSERT INTO goals
                (id, user_id, name, goal_type, target_base_minor, target_date, priority, linked_account_id)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id,
            goal.user_id,
            goal.name,
            type_str,
            goal.target_base_minor,
            goal.target_date,
            goal.priority,
            goal.linked_account_id
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Goal not found after creation"))
    }

    async fn update(
        &self,
        goal_id: Uuid,
        name: String,
        goal_type: GoalType,
        target_base_minor: i64,
        target_date: Option<NaiveDate>,
        priority: i32,
        linked_account_id: Option<Uuid>,
        user_id: Option<Uuid>
    ) -> Result<Option<Goal>, Error> {
        let type_str = goal_type.as_str();

        sqlx::query!(
            "UPDATE goals SET name = ?, goal_type = ?, target_base_minor = ?, target_date = ?, priority = ?, linked_account_id = ? WHERE id = ? AND user_id = ?",
            name, type_str, target_base_minor, target_date, priority, linked_account_id, goal_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(goal_id, user_id).await
    }

    async fn delete(&self, goal_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!("DELETE FROM goals WHERE id = ? AND user_id = ?", goal_id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Goal>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let goals = sqlx::query_as!(
            Goal,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, goal_type AS "goal_type: String",
                target_base_minor, target_date, priority,
                linked_account_id AS "linked_account_id: _",
                created_at, updated_at
            FROM goals
            WHERE user_id = ?
            ORDER BY priority DESC, created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(goals)
    }
}
