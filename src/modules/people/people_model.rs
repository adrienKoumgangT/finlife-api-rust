use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Error as SqlxError, Row};
use std::collections::HashMap;
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::modules::people::people_command::PeopleCreateCommand;
use crate::shared::db::mysql::FromSqlRow;
use crate::shared::utils::ub;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct People {
    pub id: Option<Vec<u8>>,

    pub user_id: Vec<u8>,
    pub name: String,

    pub email: Option<String>,
    pub phone: Option<String>,
    pub image_url: Option<String>,
    pub note: Option<String>,

    pub archived: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl FromSqlRow for People {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(Self {
            id: row.try_get(index_map["id"])?,
            user_id: row.try_get(index_map["user_id"])?,
            name: row.try_get(index_map["name"])?,
            email: row.try_get(index_map["email"])?,
            phone: row.try_get(index_map["phone"])?,
            image_url: row.try_get(index_map["image_url"])?,
            note: row.try_get(index_map["note"])?,
            archived: row.try_get(index_map["archived"])?,
            created_at: row.try_get(index_map["created_at"])?,
            updated_at: row.try_get(index_map["updated_at"])?,
        })
    }
}

impl From<PeopleCreateCommand> for People {
    fn from(command: PeopleCreateCommand) -> Self {
        Self {
            id: None,
            user_id: ub(command.user_id),
            name: command.people_name,
            email: command.people_email,
            phone: command.people_phone,
            image_url: command.people_image_url,
            note: command.people_note,
            archived: false,
            created_at: None,
            updated_at: None,
        }
    }
}

