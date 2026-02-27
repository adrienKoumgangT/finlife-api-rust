use chrono::{DateTime, NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::budgets::budget_model::{Budget, BudgetEnvelope, BudgetStatus, RolloverRule};

// --- BUDGET DTOs ---

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BudgetResponse {
    pub budget_id: Uuid,
    pub user_id: Uuid,
    pub month: NaiveDate,
    pub base_currency_code: String,
    pub person_id: Option<Uuid>,
    pub status: BudgetStatus,
    pub budget_created_at: Option<DateTime<Utc>>,
    pub budget_updated_at: Option<DateTime<Utc>>,
}

impl From<Budget> for BudgetResponse {
    fn from(budget: Budget) -> Self {
        Self {
            budget_id: budget.id.unwrap(),
            user_id: budget.user_id,
            month: budget.month,
            base_currency_code: budget.base_currency_code,
            person_id: budget.person_id,
            status: budget.status,
            budget_created_at: budget.created_at,
            budget_updated_at: budget.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BudgetCreateRequest {
    pub month: NaiveDate,
    pub base_currency_code: String,
    pub person_id: Option<Uuid>,
    pub status: Option<BudgetStatus>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BudgetUpdateRequest {
    pub base_currency_code: String,
    pub person_id: Option<Uuid>,
    pub status: BudgetStatus,
}

// --- ENVELOPE DTOs ---

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BudgetEnvelopeResponse {
    pub envelope_id: Uuid,
    pub budget_id: Uuid,
    pub category_id: Uuid,
    pub planned_base_minor: i64,
    pub carryover_base_minor: i64,
    pub rollover_rule: RolloverRule,
    pub envelope_created_at: Option<DateTime<Utc>>,
    pub envelope_updated_at: Option<DateTime<Utc>>,
}

impl From<BudgetEnvelope> for BudgetEnvelopeResponse {
    fn from(env: BudgetEnvelope) -> Self {
        Self {
            envelope_id: env.id.unwrap(),
            budget_id: env.budget_id,
            category_id: env.category_id,
            planned_base_minor: env.planned_base_minor,
            carryover_base_minor: env.carryover_base_minor,
            rollover_rule: env.rollover_rule,
            envelope_created_at: env.created_at,
            envelope_updated_at: env.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BudgetEnvelopeCreateRequest {
    pub budget_id: Uuid,
    pub category_id: Uuid,
    pub planned_base_minor: Option<i64>,
    pub carryover_base_minor: Option<i64>,
    pub rollover_rule: Option<RolloverRule>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BudgetEnvelopeUpdateRequest {
    pub planned_base_minor: i64,
    pub carryover_base_minor: i64,
    pub rollover_rule: RolloverRule,
}
