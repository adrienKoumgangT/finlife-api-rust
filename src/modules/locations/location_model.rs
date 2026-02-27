use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use crate::modules::locations::location_command::LocationCreateCommand;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Location {
    pub id: Option<Uuid>,
    pub user_id: Uuid,

    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,

    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,

    pub archived: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<LocationCreateCommand> for Location {
    fn from(command: LocationCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            name: command.name,
            address: command.address,
            city: command.city,
            region: command.region,
            postal_code: command.postal_code,
            country_code: command.country_code,
            latitude: command.latitude,
            longitude: command.longitude,
            archived: false,
            created_at: None,
            updated_at: None,
        }
    }
}
