use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::categories::category_command::CategoryCreateCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CategoryKind {
    Income,
    Expense,
}

impl From<String> for CategoryKind {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "income" => CategoryKind::Income,
            _ => CategoryKind::Expense,
        }
    }
}

impl CategoryKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            CategoryKind::Income => "income",
            CategoryKind::Expense => "expense",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: Option<Uuid>,
    pub user_id: Uuid,

    pub name: String,
    pub kind: CategoryKind,
    pub parent_id: Option<Uuid>,
    pub sort_order: i32,

    pub archived: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<CategoryCreateCommand> for Category {
    fn from(command: CategoryCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            name: command.name,
            kind: command.category_kind,
            parent_id: command.parent_id,
            sort_order: command.sort_order.unwrap_or(0),
            archived: false,
            created_at: None,
            updated_at: None,
        }
    }
}
