use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::projects::project_model::{Project, ProjectStatus};
use crate::shared::state::AppState;

#[async_trait]
pub trait ProjectRepositoryInterface {

    async fn get(&self, project_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Project>, Error>;

    async fn create(&self, project: Project, user_id: Option<Uuid>) -> Result<Project, Error>;

    #[allow(clippy::too_many_arguments)]
    async fn update(&self, project_id: Uuid, name: String, status: ProjectStatus, priority: i32, start_date: Option<NaiveDate>, due_date: Option<NaiveDate>, budget_base_minor: i64, goal_id: Option<Uuid>, person_id: Option<Uuid>, location_id: Option<Uuid>, description: Option<String>, user_id: Option<Uuid>) -> Result<Option<Project>, Error>;

    async fn delete(&self, project_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Project>, Error>;

}

#[derive(Clone)]
pub struct ProjectRepository {
    pool: MySqlPool,
}

impl From<&AppState> for ProjectRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl ProjectRepositoryInterface for ProjectRepository {

    async fn get(&self, project_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Project>, Error> {
        let project = sqlx::query_as!(
            Project,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, status AS "status: String", priority,
                start_date, due_date, budget_base_minor,
                goal_id AS "goal_id: _", person_id AS "person_id: _", location_id AS "location_id: _",
                description, created_at, updated_at
            FROM projects
            WHERE id = ? AND user_id = ?
            "#,
            project_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(project)
    }

    async fn create(&self, project: Project, user_id: Option<Uuid>) -> Result<Project, Error> {
        let new_id = Uuid::new_v4();
        let status_str = project.status.as_str();

        sqlx::query!(
            r#"
            INSERT INTO projects
                (id, user_id, name, status, priority, start_date, due_date, budget_base_minor, goal_id, person_id, location_id, description)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id,
            project.user_id,
            project.name,
            status_str,
            project.priority,
            project.start_date,
            project.due_date,
            project.budget_base_minor,
            project.goal_id,
            project.person_id,
            project.location_id,
            project.description
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Project not found after creation"))
    }

    async fn update(
        &self,
        project_id: Uuid,
        name: String,
        status: ProjectStatus,
        priority: i32,
        start_date: Option<NaiveDate>,
        due_date: Option<NaiveDate>,
        budget_base_minor: i64,
        goal_id: Option<Uuid>,
        person_id: Option<Uuid>,
        location_id: Option<Uuid>,
        description: Option<String>,
        user_id: Option<Uuid>
    ) -> Result<Option<Project>, Error> {
        let status_str = status.as_str();

        sqlx::query!(
            r#"
            UPDATE projects SET
                name = ?, status = ?, priority = ?, start_date = ?, due_date = ?,
                budget_base_minor = ?, goal_id = ?, person_id = ?, location_id = ?, description = ?
            WHERE id = ? AND user_id = ?
            "#,
            name, status_str, priority, start_date, due_date, budget_base_minor, goal_id, person_id, location_id, description, project_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(project_id, user_id).await
    }

    async fn delete(&self, project_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!("DELETE FROM projects WHERE id = ? AND user_id = ?", project_id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Project>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let projects = sqlx::query_as!(
            Project,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, status AS "status: String", priority,
                start_date, due_date, budget_base_minor,
                goal_id AS "goal_id: _", person_id AS "person_id: _", location_id AS "location_id: _",
                description, created_at, updated_at
            FROM projects
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

        Ok(projects)
    }
}
