use chrono::{DateTime, NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::projects::project_milestone_model::{ProjectMilestone, MilestoneStatus};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectMilestoneResponse {
    pub milestone_id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub due_date: Option<NaiveDate>,
    pub status: MilestoneStatus,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,
    pub milestone_created_at: Option<DateTime<Utc>>,
    pub milestone_updated_at: Option<DateTime<Utc>>,
}

impl From<ProjectMilestone> for ProjectMilestoneResponse {
    fn from(milestone: ProjectMilestone) -> Self {
        Self {
            milestone_id: milestone.id.unwrap(),
            project_id: milestone.project_id,
            title: milestone.title,
            due_date: milestone.due_date,
            status: milestone.status,
            person_id: milestone.person_id,
            location_id: milestone.location_id,
            note: milestone.note,
            milestone_created_at: milestone.created_at,
            milestone_updated_at: milestone.updated_at,
        }
    }
}

impl From<&ProjectMilestone> for ProjectMilestoneResponse {
    fn from(milestone: &ProjectMilestone) -> Self {
        Self {
            milestone_id: milestone.id.clone().unwrap(),
            project_id: milestone.project_id.clone(),
            title: milestone.title.clone(),
            due_date: milestone.due_date.clone(),
            status: milestone.status.clone(),
            person_id: milestone.person_id.clone(),
            location_id: milestone.location_id.clone(),
            note: milestone.note.clone(),
            milestone_created_at: milestone.created_at.clone(),
            milestone_updated_at: milestone.updated_at.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectMilestoneCreateRequest {
    pub project_id: Uuid,
    pub title: String,
    pub due_date: Option<NaiveDate>,
    pub status: Option<MilestoneStatus>,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectMilestoneUpdateRequest {
    pub title: String,
    pub due_date: Option<NaiveDate>,
    pub status: MilestoneStatus,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,
}
