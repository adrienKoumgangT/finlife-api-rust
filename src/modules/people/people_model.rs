use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::modules::people::people_command::PeopleCreateCommand;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct People {
    pub id: Option<Uuid>,
    pub user_id: Uuid,

    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub image_url: Option<String>,
    pub note: Option<String>,

    pub archived: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<PeopleCreateCommand> for People {
    fn from(command: PeopleCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            name: command.name,
            email: command.email,
            phone: command.phone,
            image_url: command.image_url,
            note: command.note,
            archived: false,
            created_at: None,
            updated_at: None,
        }
    }
}
