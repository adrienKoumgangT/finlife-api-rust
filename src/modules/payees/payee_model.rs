use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::modules::payees::payee_command::PayeeCreateCommand;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Payee {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<PayeeCreateCommand> for Payee {
    fn from(command: PayeeCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            name: command.name,
            created_at: None,
        }
    }
}
