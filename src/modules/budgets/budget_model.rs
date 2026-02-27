use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::budgets::budget_command::{BudgetCreateCommand, BudgetEnvelopeCreateCommand};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum BudgetStatus {
    Draft,
    Active,
    Closed,
}

impl From<String> for BudgetStatus {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "draft" => BudgetStatus::Draft,
            "closed" => BudgetStatus::Closed,
            _ => BudgetStatus::Active,
        }
    }
}

impl BudgetStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            BudgetStatus::Draft => "draft",
            BudgetStatus::Active => "active",
            BudgetStatus::Closed => "closed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RolloverRule {
    None,
    Full,
    Partial,
}

impl From<String> for RolloverRule {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "none" => RolloverRule::None,
            "partial" => RolloverRule::Partial,
            _ => RolloverRule::Full,
        }
    }
}

impl RolloverRule {
    pub fn as_str(&self) -> &'static str {
        match self {
            RolloverRule::None => "none",
            RolloverRule::Full => "full",
            RolloverRule::Partial => "partial",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Budget {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub month: NaiveDate,
    pub base_currency_code: String,
    pub person_id: Option<Uuid>,
    pub status: BudgetStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<BudgetCreateCommand> for Budget {
    fn from(command: BudgetCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            month: command.month,
            base_currency_code: command.base_currency_code,
            person_id: command.person_id,
            status: command.status.unwrap_or(BudgetStatus::Active),
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BudgetEnvelope {
    pub id: Option<Uuid>,
    pub budget_id: Uuid,
    pub category_id: Uuid,
    pub planned_base_minor: i64,
    pub carryover_base_minor: i64,
    pub rollover_rule: RolloverRule,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<BudgetEnvelopeCreateCommand> for BudgetEnvelope {
    fn from(command: BudgetEnvelopeCreateCommand) -> Self {
        Self {
            id: None,
            budget_id: command.budget_id,
            category_id: command.category_id,
            planned_base_minor: command.planned_base_minor.unwrap_or(0),
            carryover_base_minor: command.carryover_base_minor.unwrap_or(0),
            rollover_rule: command.rollover_rule.unwrap_or(RolloverRule::Full),
            created_at: None,
            updated_at: None,
        }
    }
}
