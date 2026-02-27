use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::projects::project_task_model::{ProjectTask, TaskStatus};
use crate::shared::state::AppState;

#[async_trait]
pub trait ProjectTaskRepositoryInterface {

    async fn get(&self, task_id: Uuid, user_id: Option<Uuid>) -> Result<Option<ProjectTask>, Error>;

    async fn create(&self, task: ProjectTask, user_id: Option<Uuid>) -> Result<ProjectTask, Error>;

    #[allow(clippy::too_many_arguments)]
    async fn update(&self, task_id: Uuid, title: String, status: TaskStatus, due_date: Option<NaiveDate>, parent_task_id: Option<Uuid>, order_idx: i32, estimate_minutes: Option<i32>, actual_minutes: Option<i32>, assigned_person_id: Option<Uuid>, location_id: Option<Uuid>, note: Option<String>, user_id: Option<Uuid>) -> Result<Option<ProjectTask>, Error>;

    async fn delete(&self, task_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_by_project(&self, project_id: Uuid, limit: Option<u32>, offset: Option<u32>, user_id: Option<Uuid>) -> Result<Vec<ProjectTask>, Error>;

}

#[derive(Clone)]
pub struct ProjectTaskRepository {
    pool: MySqlPool,
}

impl From<&AppState> for ProjectTaskRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl ProjectTaskRepositoryInterface for ProjectTaskRepository {

    async fn get(&self, task_id: Uuid, user_id: Option<Uuid>) -> Result<Option<ProjectTask>, Error> {
        let task = sqlx::query_as!(
            ProjectTask,
            r#"
            SELECT
                t.id AS "id: _", t.project_id AS "project_id: _",
                t.title, t.status AS "status: String", t.due_date,
                t.parent_task_id AS "parent_task_id: _", t.order_idx,
                t.estimate_minutes, t.actual_minutes,
                t.assigned_person_id AS "assigned_person_id: _", t.location_id AS "location_id: _",
                t.note, t.created_at, t.updated_at
            FROM project_tasks t
            JOIN projects p ON p.id = t.project_id
            WHERE t.id = ? AND p.user_id = ?
            "#,
            task_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(task)
    }

    async fn create(&self, task: ProjectTask, user_id: Option<Uuid>) -> Result<ProjectTask, Error> {
        let project_exists = sqlx::query!("SELECT id FROM projects WHERE id = ? AND user_id = ?", task.project_id, user_id)
            .fetch_optional(&self.pool)
            .await?;

        if project_exists.is_none() {
            return Err(Error::msg("Unauthorized or project not found"));
        }

        let new_id = Uuid::new_v4();
        let status_str = task.status.as_str();

        sqlx::query!(
            r#"
            INSERT INTO project_tasks
                (id, project_id, title, status, due_date, parent_task_id, order_idx, estimate_minutes, actual_minutes, assigned_person_id, location_id, note)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id, task.project_id, task.title, status_str, task.due_date, task.parent_task_id, task.order_idx, task.estimate_minutes, task.actual_minutes, task.assigned_person_id, task.location_id, task.note
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Task not found after creation"))
    }

    async fn update(
        &self,
        task_id: Uuid,
        title: String,
        status: TaskStatus,
        due_date: Option<NaiveDate>,
        parent_task_id: Option<Uuid>,
        order_idx: i32,
        estimate_minutes: Option<i32>,
        actual_minutes: Option<i32>,
        assigned_person_id: Option<Uuid>,
        location_id: Option<Uuid>,
        note: Option<String>,
        user_id: Option<Uuid>
    ) -> Result<Option<ProjectTask>, Error> {
        let status_str = status.as_str();

        sqlx::query!(
            r#"
            UPDATE project_tasks t
            JOIN projects p ON p.id = t.project_id
            SET
                t.title = ?, t.status = ?, t.due_date = ?, t.parent_task_id = ?,
                t.order_idx = ?, t.estimate_minutes = ?, t.actual_minutes = ?,
                t.assigned_person_id = ?, t.location_id = ?, t.note = ?
            WHERE t.id = ? AND p.user_id = ?
            "#,
            title, status_str, due_date, parent_task_id, order_idx, estimate_minutes, actual_minutes, assigned_person_id, location_id, note, task_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(task_id, user_id).await
    }

    async fn delete(&self, task_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!(
            "DELETE t FROM project_tasks t JOIN projects p ON p.id = t.project_id WHERE t.id = ? AND p.user_id = ?",
            task_id, user_id
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_project(&self, project_id: Uuid, limit: Option<u32>, offset: Option<u32>, user_id: Option<Uuid>) -> Result<Vec<ProjectTask>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let tasks = sqlx::query_as!(
            ProjectTask,
            r#"
            SELECT
                t.id AS "id: _", t.project_id AS "project_id: _",
                t.title, t.status AS "status: String", t.due_date,
                t.parent_task_id AS "parent_task_id: _", t.order_idx,
                t.estimate_minutes, t.actual_minutes,
                t.assigned_person_id AS "assigned_person_id: _", t.location_id AS "location_id: _",
                t.note, t.created_at, t.updated_at
            FROM project_tasks t
            JOIN projects p ON p.id = t.project_id
            WHERE t.project_id = ? AND p.user_id = ?
            ORDER BY t.order_idx ASC, t.created_at ASC
            LIMIT ? OFFSET ?
            "#,
            project_id, user_id, limit_val, offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(tasks)
    }

}
