use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::goals::goal_command::GoalCreateCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum GoalType {
    Savings,
    Debt,
    Investment,
    OneShot,
}

impl From<String> for GoalType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "debt" => GoalType::Debt,
            "investment" => GoalType::Investment,
            "one_shot" => GoalType::OneShot,
            _ => GoalType::Savings,
        }
    }
}

impl GoalType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GoalType::Savings => "savings",
            GoalType::Debt => "debt",
            GoalType::Investment => "investment",
            GoalType::OneShot => "one_shot",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Goal {
    pub id: Option<Uuid>,
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

impl From<GoalCreateCommand> for Goal {
    fn from(command: GoalCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            name: command.name,
            goal_type: command.goal_type,
            target_base_minor: command.target_base_minor.unwrap_or(0),
            target_date: command.target_date,
            priority: command.priority.unwrap_or(0),
            linked_account_id: command.linked_account_id,
            created_at: None,
            updated_at: None,
        }
    }
}
