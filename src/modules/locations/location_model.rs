use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Error as SqlxError, Row};
use std::collections::HashMap;
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::modules::locations::location_command::LocationCreateCommand;
use crate::shared::db::mysql::FromSqlRow;
use crate::shared::utils::ub;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Location {
    pub id: Option<Vec<u8>>,

    pub user_id: Vec<u8>,
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

impl FromSqlRow for Location {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(Self {
            id: row.try_get(index_map["id"])?,
            user_id: row.try_get(index_map["user_id"])?,
            name: row.try_get(index_map["name"])?,
            address: row.try_get(index_map["address"])?,
            city: row.try_get(index_map["city"])?,
            region: row.try_get(index_map["region"])?,
            postal_code: row.try_get(index_map["postal_code"])?,
            country_code: row.try_get(index_map["country_code"])?,
            latitude: row.try_get(index_map["latitude"])?,
            longitude: row.try_get(index_map["longitude"])?,
            archived: row.try_get(index_map["archived"])?,
            created_at: row.try_get(index_map["created_at"])?,
            updated_at: row.try_get(index_map["updated_at"])?,
        })
    }
}

impl From<LocationCreateCommand> for Location {
    fn from(command: LocationCreateCommand) -> Self {
        Self {
            id: None,
            user_id: ub(command.user_id),
            name: command.location_name,
            address: command.location_address,
            city: command.location_city,
            region: command.location_region,
            postal_code: command.location_postal_code,
            country_code: command.location_country_code,
            latitude: command.location_latitude,
            longitude: command.location_longitude,
            archived: false,
            created_at: None,
            updated_at: None,
        }
    }
}
