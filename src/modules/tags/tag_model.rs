use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::modules::tags::tag_command::TagCreateCommand;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: Option<Uuid>,
    pub user_id: Uuid,

    pub name: String,

    pub created_at: Option<DateTime<Utc>>,
}

impl From<TagCreateCommand> for Tag {
    fn from(command: TagCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            name: command.name,
            created_at: None
        }
    }
}

