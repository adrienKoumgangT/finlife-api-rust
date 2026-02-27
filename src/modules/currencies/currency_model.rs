use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use crate::modules::currencies::currency_command::{CurrencyCreateCommand, FxRateCreateCommand};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Currency {
    pub code: String,
    pub name: String,

    pub minor_unit: u8,
}

impl From<CurrencyCreateCommand> for Currency {
    fn from(command: CurrencyCreateCommand) -> Self {
        Self {
            code: command.code,
            name: command.name,
            minor_unit: command.minor_unit
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FxRate {
    pub id: Option<Uuid>,

    pub base_code: String,
    pub quote_code: String,
    pub rate: Decimal,
    pub as_of_date: NaiveDate,
    pub source: String,

    pub created_at: Option<DateTime<Utc>>,
}


impl From<FxRateCreateCommand> for FxRate {
    fn from(command: FxRateCreateCommand) -> Self {
        Self {
            id: None,
            base_code: command.base_code,
            quote_code: command.quote_code,
            rate: command.rate,
            as_of_date: command.as_of_date,
            source: command.source,
            created_at: None
        }
    }
}
