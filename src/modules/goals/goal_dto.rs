use chrono::{DateTime, NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::goals::goal_model::{Goal, GoalType};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GoalResponse {
    pub goal_id: Uuid,
    pub user_id: Uuid,

    pub name: String,
    pub goal_type: GoalType,
    pub target_base_minor: i64,
    pub target_date: Option<NaiveDate>,
    pub priority: i32,
    pub linked_account_id: Option<Uuid>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Goal> for GoalResponse {
    fn from(goal: Goal) -> Self {
        Self {
            goal_id: goal.id.unwrap(),
            user_id: goal.user_id,
            name: goal.name,
            goal_type: goal.goal_type,
            target_base_minor: goal.target_base_minor,
            target_date: goal.target_date,
            priority: goal.priority,
            linked_account_id: goal.linked_account_id,
            created_at: goal.created_at,
            updated_at: goal.updated_at,
        }
    }
}

impl From<&Goal> for GoalResponse {
    fn from(goal: &Goal) -> Self {
        Self {
            goal_id: goal.id.clone().unwrap(),
            user_id: goal.user_id.clone(),
            name: goal.name.clone(),
            goal_type: goal.goal_type.clone(),
            target_base_minor: goal.target_base_minor,
            target_date: goal.target_date.clone(),
            priority: goal.priority,
            linked_account_id: goal.linked_account_id.clone(),
            created_at: goal.created_at.clone(),
            updated_at: goal.updated_at.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GoalCreateRequest {
    pub name: String,
    pub goal_type: GoalType,
    pub target_base_minor: Option<i64>,
    pub target_date: Option<NaiveDate>,
    pub priority: Option<i32>,
    pub linked_account_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GoalUpdateRequest {
    pub name: String,
    pub goal_type: GoalType,
    pub target_base_minor: Option<i64>,
    pub target_date: Option<NaiveDate>,
    pub priority: Option<i32>,
    pub linked_account_id: Option<Uuid>,
}
