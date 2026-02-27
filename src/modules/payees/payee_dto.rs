use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::payees::payee_model::Payee;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PayeeResponse {
    pub payee_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<Payee> for PayeeResponse {
    fn from(payee: Payee) -> Self {
        Self {
            payee_id: payee.id.unwrap(),
            user_id: payee.user_id,
            name: payee.name,
            created_at: payee.created_at,
        }
    }
}

impl From<&Payee> for PayeeResponse {
    fn from(payee: &Payee) -> Self {
        Self {
            payee_id: payee.id.clone().unwrap(),
            user_id: payee.user_id.clone(),
            name: payee.name.clone(),
            created_at: payee.created_at.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PayeeCreateRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PayeeUpdateRequest {
    pub name: String,
}
