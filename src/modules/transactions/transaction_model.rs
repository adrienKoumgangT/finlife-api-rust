use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::transactions::transaction_command::TransactionCreateCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Pending,
    Cleared,
}

impl From<String> for TransactionStatus {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "pending" => TransactionStatus::Pending,
            _ => TransactionStatus::Cleared, // Default fallback
        }
    }
}

impl TransactionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionStatus::Pending => "pending",
            TransactionStatus::Cleared => "cleared",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub occurred_at: NaiveDateTime,

    pub amount_minor: i64,
    pub currency_code: String,
    pub base_amount_minor: i64,
    pub base_currency_code: String,

    pub fx_rate_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub payee_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,

    pub note: Option<String>,
    pub project_id: Option<Uuid>,
    pub goal_id: Option<Uuid>,

    pub status: TransactionStatus,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<TransactionCreateCommand> for Transaction {
    fn from(command: TransactionCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            account_id: command.account_id,
            occurred_at: command.occurred_at,
            amount_minor: command.amount_minor,
            currency_code: command.currency_code,
            base_amount_minor: command.base_amount_minor,
            base_currency_code: command.base_currency_code,
            fx_rate_id: command.fx_rate_id,
            category_id: command.category_id,
            payee_id: command.payee_id,
            person_id: command.person_id,
            location_id: command.location_id,
            note: command.note,
            project_id: command.project_id,
            goal_id: command.goal_id,
            status: command.status.unwrap_or(TransactionStatus::Cleared),
            created_at: None,
            updated_at: None,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MonthlyFlow {
    pub month: Option<String>,       // e.g., "2025-04"
    pub total_income: i64,   // Minor units
    pub total_expense: i64,  // Minor units
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MonthlyCategoryExpense {
    pub month: Option<String>,
    pub category_id: Option<Uuid>,
    pub total_amount: i64,
}


