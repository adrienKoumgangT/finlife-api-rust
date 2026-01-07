use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Error as SqlxError, Row};
use std::collections::HashMap;
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::modules::currencies::currency_command::{CurrencyCreateCommand, FxRateCreateCommand};
use crate::shared::db::mysql::FromSqlRow;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Currency {
    pub code: String,
    pub name: String,

    pub minor_unit: u8,
}

impl FromSqlRow for Currency {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(Self {
            code: row.try_get(index_map["code"])?,
            name: row.try_get(index_map["name"])?,
            minor_unit: row.try_get(index_map["minor_unit"])?,
        })
    }
}

impl From<CurrencyCreateCommand> for Currency {
    fn from(command: CurrencyCreateCommand) -> Self {
        Self {
            code: command.currency_code,
            name: command.currency_name,
            minor_unit: command.currency_minor_unit
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FxRate {
    pub id: Option<Vec<u8>>,

    pub base_code: String,
    pub quote_code: String,
    pub rate: Decimal,
    pub as_of_date: NaiveDate,
    pub source: String,

    pub created_at: Option<DateTime<Utc>>,
}

impl FromSqlRow for FxRate {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(Self {
            id: row.try_get(index_map["id"])?,
            base_code: row.try_get(index_map["base_code"])?,
            quote_code: row.try_get(index_map["quote_code"])?,
            rate: row.try_get(index_map["rate"])?,
            as_of_date: row.try_get(index_map["as_of_date"])?,
            source: row.try_get(index_map["source"])?,
            created_at: row.try_get(index_map["created_at"])?,
        })
    }
}

impl From<FxRateCreateCommand> for FxRate {
    fn from(command: FxRateCreateCommand) -> Self {
        Self {
            id: None,
            base_code: command.fx_rate_base_code,
            quote_code: command.fx_rate_quote_code,
            rate: command.fx_rate_rate,
            as_of_date: command.fx_rate_as_of_date,
            source: command.fx_rate_source,
            created_at: None
        }
    }
}
