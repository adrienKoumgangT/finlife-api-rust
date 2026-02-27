use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::budgets::budget_dto::*;
use crate::modules::budgets::budget_model::{BudgetStatus, RolloverRule};
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

// ==========================================
//                 BUDGETS
// ==========================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetGetCommand {
    pub budget_id: Uuid,
    pub auth_user: AuthUser,
}

impl BudgetGetCommand {
    pub fn new(budget_id: Uuid, auth_user: AuthUser) -> Self {
        Self { budget_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetCreateCommand {
    pub user_id: Uuid,
    pub month: NaiveDate,
    pub base_currency_code: String,
    pub person_id: Option<Uuid>,
    pub status: Option<BudgetStatus>,
    pub auth_user: AuthUser,
}

impl BudgetCreateCommand {
    pub fn new(request: BudgetCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id.clone(),
            month: request.month,
            base_currency_code: request.base_currency_code,
            person_id: request.person_id,
            status: request.status,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetUpdateCommand {
    pub budget_id: Uuid,
    pub base_currency_code: String,
    pub person_id: Option<Uuid>,
    pub status: BudgetStatus,
    pub auth_user: AuthUser,
}

impl BudgetUpdateCommand {
    pub fn new(budget_id: Uuid, request: BudgetUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            budget_id,
            base_currency_code: request.base_currency_code,
            person_id: request.person_id,
            status: request.status,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetDeleteCommand {
    pub budget_id: Uuid,
    pub auth_user: AuthUser,
}

impl BudgetDeleteCommand {
    pub fn new(budget_id: Uuid, auth_user: AuthUser) -> Self {
        Self { budget_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl BudgetListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}


// ==========================================
//             BUDGET ENVELOPES
// ==========================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetEnvelopeGetCommand {
    pub envelope_id: Uuid,
    pub auth_user: AuthUser,
}

impl BudgetEnvelopeGetCommand {
    pub fn new(envelope_id: Uuid, auth_user: AuthUser) -> Self {
        Self { envelope_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetEnvelopeCreateCommand {
    pub budget_id: Uuid,
    pub category_id: Uuid,
    pub planned_base_minor: Option<i64>,
    pub carryover_base_minor: Option<i64>,
    pub rollover_rule: Option<RolloverRule>,
    pub auth_user: AuthUser,
}

impl BudgetEnvelopeCreateCommand {
    pub fn new(request: BudgetEnvelopeCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            budget_id: request.budget_id,
            category_id: request.category_id,
            planned_base_minor: request.planned_base_minor,
            carryover_base_minor: request.carryover_base_minor,
            rollover_rule: request.rollover_rule,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetEnvelopeUpdateCommand {
    pub envelope_id: Uuid,
    pub planned_base_minor: i64,
    pub carryover_base_minor: i64,
    pub rollover_rule: RolloverRule,
    pub auth_user: AuthUser,
}

impl BudgetEnvelopeUpdateCommand {
    pub fn new(envelope_id: Uuid, request: BudgetEnvelopeUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            envelope_id,
            planned_base_minor: request.planned_base_minor,
            carryover_base_minor: request.carryover_base_minor,
            rollover_rule: request.rollover_rule,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetEnvelopeDeleteCommand {
    pub envelope_id: Uuid,
    pub auth_user: AuthUser,
}

impl BudgetEnvelopeDeleteCommand {
    pub fn new(envelope_id: Uuid, auth_user: AuthUser) -> Self {
        Self { envelope_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetEnvelopeListByBudgetCommand {
    pub budget_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl BudgetEnvelopeListByBudgetCommand {
    pub fn new(budget_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { budget_id, pagination, auth_user }
    }
}
