use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::projects::project_milestone_model::{ProjectMilestone, MilestoneStatus};
use crate::shared::state::AppState;

#[async_trait]
pub trait ProjectMilestoneRepositoryInterface {

    async fn get(&self, milestone_id: Uuid, user_id: Option<Uuid>) -> Result<Option<ProjectMilestone>, Error>;

    async fn create(&self, milestone: ProjectMilestone, user_id: Option<Uuid>) -> Result<ProjectMilestone, Error>;

    #[allow(clippy::too_many_arguments)]
    async fn update(&self, milestone_id: Uuid, title: String, due_date: Option<NaiveDate>, status: MilestoneStatus, person_id: Option<Uuid>, location_id: Option<Uuid>, note: Option<String>, user_id: Option<Uuid>) -> Result<Option<ProjectMilestone>, Error>;

    async fn delete(&self, milestone_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_by_project(&self, project_id: Uuid, limit: Option<u32>, offset: Option<u32>, user_id: Option<Uuid>) -> Result<Vec<ProjectMilestone>, Error>;

}

#[derive(Clone)]
pub struct ProjectMilestoneRepository {
    pool: MySqlPool,
}

impl From<&AppState> for ProjectMilestoneRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl ProjectMilestoneRepositoryInterface for ProjectMilestoneRepository {

    async fn get(&self, milestone_id: Uuid, user_id: Option<Uuid>) -> Result<Option<ProjectMilestone>, Error> {
        let milestone = sqlx::query_as!(
            ProjectMilestone,
            r#"
            SELECT
                m.id AS "id: _", m.project_id AS "project_id: _",
                m.title, m.due_date, m.status AS "status: String",
                m.person_id AS "person_id: _", m.location_id AS "location_id: _",
                m.note, m.created_at, m.updated_at
            FROM project_milestones m
            JOIN projects p ON p.id = m.project_id
            WHERE m.id = ? AND p.user_id = ?
            "#,
            milestone_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(milestone)
    }

    async fn create(&self, milestone: ProjectMilestone, user_id: Option<Uuid>) -> Result<ProjectMilestone, Error> {
        let project_exists = sqlx::query!("SELECT id FROM projects WHERE id = ? AND user_id = ?", milestone.project_id, user_id)
            .fetch_optional(&self.pool)
            .await?;

        if project_exists.is_none() {
            return Err(Error::msg("Unauthorized or project not found"));
        }

        let new_id = Uuid::new_v4();
        let status_str = milestone.status.as_str();

        sqlx::query!(
            r#"
            INSERT INTO project_milestones
                (id, project_id, title, due_date, status, person_id, location_id, note)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id, milestone.project_id, milestone.title, milestone.due_date, status_str, milestone.person_id, milestone.location_id, milestone.note
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Milestone not found after creation"))
    }

    async fn update(
        &self,
        milestone_id: Uuid,
        title: String,
        due_date: Option<NaiveDate>,
        status: MilestoneStatus,
        person_id: Option<Uuid>,
        location_id: Option<Uuid>,
        note: Option<String>,
        user_id: Option<Uuid>
    ) -> Result<Option<ProjectMilestone>, Error> {
        let status_str = status.as_str();

        sqlx::query!(
            r#"
            UPDATE project_milestones m
            JOIN projects p ON p.id = m.project_id
            SET
                m.title = ?, m.due_date = ?, m.status = ?,
                m.person_id = ?, m.location_id = ?, m.note = ?
            WHERE m.id = ? AND p.user_id = ?
            "#,
            title, due_date, status_str, person_id, location_id, note, milestone_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(milestone_id, user_id).await
    }

    async fn delete(&self, milestone_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!(
            "DELETE m FROM project_milestones m JOIN projects p ON p.id = m.project_id WHERE m.id = ? AND p.user_id = ?",
            milestone_id, user_id
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_project(&self, project_id: Uuid, limit: Option<u32>, offset: Option<u32>, user_id: Option<Uuid>) -> Result<Vec<ProjectMilestone>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let milestones = sqlx::query_as!(
            ProjectMilestone,
            r#"
            SELECT
                m.id AS "id: _", m.project_id AS "project_id: _",
                m.title, m.due_date, m.status AS "status: String",
                m.person_id AS "person_id: _", m.location_id AS "location_id: _",
                m.note, m.created_at, m.updated_at
            FROM project_milestones m
            JOIN projects p ON p.id = m.project_id
            WHERE m.project_id = ? AND p.user_id = ?
            ORDER BY m.due_date ASC, m.created_at ASC
            LIMIT ? OFFSET ?
            "#,
            project_id, user_id, limit_val, offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(milestones)
    }
}
