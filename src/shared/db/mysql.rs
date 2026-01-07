use std::collections::HashMap;
use chrono::{DateTime, Utc};
use anyhow::{Error, Result};
use async_trait::async_trait;
use rust_decimal::Decimal;
use sqlx::{
    mysql::MySqlRow, Column, MySqlPool, Row, MySql, Pool,
    types::{
        chrono::{NaiveDate, NaiveDateTime},
        Uuid,
    }
};
use tracing::info;

use crate::shared::config::AppDatabaseMySQLConfig;
use crate::shared::log::TimePrinter;


pub async fn connect(database_config: &AppDatabaseMySQLConfig) -> Result<Pool<MySql>> {
    info!("Connecting to MySQL database...");

    let pool = MySqlPool::connect(database_config.uri.as_str()).await?;

    // Run migrations
    // run_migrations(&pool).await?;

    info!("MySQL database connected successfully");
    Ok(pool)
}


pub fn get_column_index_map(row: &MySqlRow) -> HashMap<String, usize> {
    row.columns().iter()
        .enumerate()
        .map(|(i, col)| (col.name().to_lowercase(), i))
        .collect()
}


pub async fn run_migrations(pool: &Pool<MySql>) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    Ok(())
}


/// Trait for entities that can be mapped from database rows
pub trait FromSqlRow: Sized {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, sqlx::Error>;
}

/// Generic repository operations
#[async_trait]
pub trait GenericRepository<T: FromSqlRow + Send + Sync> {
    fn get_pool(&self) -> &MySqlPool;

    // Generic parsing methods
    fn parse_entity_from_option_result_sql(
        &self,
        timer_printer: TimePrinter,
        entity_sql_result: Result<Option<MySqlRow>, Error>,
    ) -> Result<Option<T>, Error> {
        match entity_sql_result {
            Ok(entity_opt_row) => {
                match entity_opt_row {
                    Some(entity_row) => {
                        let index_map = get_column_index_map(&entity_row);
                        let entity_result = T::map_row_to_entity(entity_row, &index_map);

                        match entity_result {
                            Ok(entity) => {
                                timer_printer.log();
                                Ok(Some(entity))
                            },
                            Err(e) => {
                                timer_printer.error_with_message(&format!("Failed to map row to entity: {}", e));
                                Err(Error::msg("Failed to convert row to entity model"))
                            }
                        }
                    },
                    None => {
                        timer_printer.warning_with_message("No entity found.");
                        Ok(None)
                    }
                }
            }
            Err(e) => {
                timer_printer.error_with_message(&format!("Error during database operation: {}", e));
                Err(Error::msg("Error during database operation"))
            }
        }
    }

    fn parse_entity_from_result_sql(
        &self,
        timer_printer: TimePrinter,
        entity_sql_row: Result<MySqlRow, Error>,
    ) -> Result<T, Error> {
        match entity_sql_row {
            Ok(entity_row) => {
                let index_map = get_column_index_map(&entity_row);
                let entity = T::map_row_to_entity(entity_row, &index_map);

                match entity {
                    Ok(entity) => {
                        timer_printer.log();
                        Ok(entity)
                    },
                    Err(e) => {
                        timer_printer.error_with_message(&format!("Failed to map row to entity: {}", e));
                        Err(Error::msg("Failed to convert row to entity model"))
                    }
                }
            },
            Err(e) => {
                timer_printer.error_with_message(&format!("Failed database operation: {}", e));
                Err(Error::msg("Failed database operation"))
            }
        }
    }

    fn parse_entity_from_result_sql_list(
        &self,
        time_printer: TimePrinter,
        result_sql_rows: Result<Vec<MySqlRow>, Error>,
    ) -> Result<Vec<T>, Error> {
        match result_sql_rows {
            Ok(entity_rows) => {
                if entity_rows.is_empty() {
                    return Ok(vec![]);
                }

                let index_map = get_column_index_map(&entity_rows[0]);
                let mut entities = Vec::with_capacity(entity_rows.len());

                for row in entity_rows {
                    match T::map_row_to_entity(row, &index_map) {
                        Ok(entity) => entities.push(entity),
                        Err(e) => {
                            time_printer.error_with_message(&format!("Failed to map row to entity: {}", e));
                            return Err(Error::msg(format!("Failed to map row to entity: {}", e)));
                        }
                    }
                }

                time_printer.log();
                Ok(entities)
            }
            Err(e) => {
                time_printer.error_with_message(&format!("Failed database operation: {}", e));
                Err(Error::msg("Failed database operation"))
            }
        }
    }
}





/// Dynamic parameter type for MySQL queries.
/// Extend with other variants if you need more types.
#[derive(Debug, Clone)]
pub enum MySqlParam {
    // ----- Non-null primitives -----
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),

    // ----- Nullable primitives -----
    OptI8(Option<i8>),
    OptI16(Option<i16>),
    OptI32(Option<i32>),
    OptI64(Option<i64>),
    OptU8(Option<u8>),
    OptU16(Option<u16>),
    OptU32(Option<u32>),
    OptU64(Option<u64>),
    OptF32(Option<f32>),
    OptF64(Option<f64>),
    OptBool(Option<bool>),
    OptString(Option<String>),
    OptBytes(Option<Vec<u8>>),

    // ----- Chrono date/time (MySQL DATE / DATETIME) -----
    NaiveDate(NaiveDate),
    NaiveDateTime(NaiveDateTime),
    DateTimeUtc(DateTime<Utc>),
    OptNaiveDate(Option<NaiveDate>),
    OptNaiveDateTime(Option<NaiveDateTime>),
    OptDateTimeUtc(Option<DateTime<Utc>>),

    // ----- UUID (BINARY(16) / CHAR(36)) -----
    Uuid(Uuid),
    OptUuid(Option<Uuid>),
    
    Decimal(Decimal),
    OptDecimal(Option<Decimal>),
}

impl MySqlParam {
    /// Bind this parameter to a `sqlx` query.
    pub fn bind<'q>(
        self,
        query: sqlx::query::Query<'q, MySql, sqlx::mysql::MySqlArguments>,
    ) -> sqlx::query::Query<'q, MySql, sqlx::mysql::MySqlArguments> {
        match self {
            // ---- non-null ----
            MySqlParam::I8(v) => query.bind(v),
            MySqlParam::I16(v) => query.bind(v),
            MySqlParam::I32(v) => query.bind(v),
            MySqlParam::I64(v) => query.bind(v),
            MySqlParam::U8(v) => query.bind(v),
            MySqlParam::U16(v) => query.bind(v),
            MySqlParam::U32(v) => query.bind(v),
            MySqlParam::U64(v) => query.bind(v),
            MySqlParam::F32(v) => query.bind(v),
            MySqlParam::F64(v) => query.bind(v),
            MySqlParam::Bool(v) => query.bind(v),
            MySqlParam::String(v) => query.bind(v),
            MySqlParam::Bytes(v) => query.bind(v),

            // ---- nullable ----
            MySqlParam::OptI8(v) => query.bind(v),
            MySqlParam::OptI16(v) => query.bind(v),
            MySqlParam::OptI32(v) => query.bind(v),
            MySqlParam::OptI64(v) => query.bind(v),
            MySqlParam::OptU8(v) => query.bind(v),
            MySqlParam::OptU16(v) => query.bind(v),
            MySqlParam::OptU32(v) => query.bind(v),
            MySqlParam::OptU64(v) => query.bind(v),
            MySqlParam::OptF32(v) => query.bind(v),
            MySqlParam::OptF64(v) => query.bind(v),
            MySqlParam::OptBool(v) => query.bind(v),
            MySqlParam::OptString(v) => query.bind(v),
            MySqlParam::OptBytes(v) => query.bind(v),

            // ---- chrono ----
            MySqlParam::NaiveDate(v) => query.bind(v),
            MySqlParam::NaiveDateTime(v) => query.bind(v),
            MySqlParam::DateTimeUtc(v) => query.bind(v),
            MySqlParam::OptNaiveDate(v) => query.bind(v),
            MySqlParam::OptNaiveDateTime(v) => query.bind(v),
            MySqlParam::OptDateTimeUtc(v) => query.bind(v),

            // ---- uuid ----
            MySqlParam::Uuid(v) => query.bind(v),
            MySqlParam::OptUuid(v) => query.bind(v),
            
            MySqlParam::Decimal(v) => query.bind(v),
            MySqlParam::OptDecimal(v) => query.bind(v),
        }
    }
}

//
// Ergonomic From impls so you can just do `42.into()`, `"foo".into()`, `Some(1).into()`, etc.
//

// ---- non-null ints ----
impl From<i8> for MySqlParam {
    fn from(v: i8) -> Self {
        MySqlParam::I8(v)
    }
}
impl From<i16> for MySqlParam {
    fn from(v: i16) -> Self {
        MySqlParam::I16(v)
    }
}
impl From<i32> for MySqlParam {
    fn from(v: i32) -> Self {
        MySqlParam::I32(v)
    }
}
impl From<i64> for MySqlParam {
    fn from(v: i64) -> Self {
        MySqlParam::I64(v)
    }
}
impl From<u8> for MySqlParam {
    fn from(v: u8) -> Self {
        MySqlParam::U8(v)
    }
}
impl From<u16> for MySqlParam {
    fn from(v: u16) -> Self {
        MySqlParam::U16(v)
    }
}
impl From<u32> for MySqlParam {
    fn from(v: u32) -> Self {
        MySqlParam::U32(v)
    }
}
impl From<u64> for MySqlParam {
    fn from(v: u64) -> Self {
        MySqlParam::U64(v)
    }
}

// ---- floats ----
impl From<f32> for MySqlParam {
    fn from(v: f32) -> Self {
        MySqlParam::F32(v)
    }
}
impl From<f64> for MySqlParam {
    fn from(v: f64) -> Self {
        MySqlParam::F64(v)
    }
}

// ---- bool ----
impl From<bool> for MySqlParam {
    fn from(v: bool) -> Self {
        MySqlParam::Bool(v)
    }
}

// ---- strings ----
impl From<String> for MySqlParam {
    fn from(v: String) -> Self {
        MySqlParam::String(v)
    }
}
impl From<&String> for MySqlParam {
    fn from(v: &String) -> Self {
        MySqlParam::String(v.clone())
    }
}
impl From<&str> for MySqlParam {
    fn from(v: &str) -> Self {
        MySqlParam::String(v.to_owned())
    }
}

// ---- bytes ----
impl From<Vec<u8>> for MySqlParam {
    fn from(v: Vec<u8>) -> Self {
        MySqlParam::Bytes(v)
    }
}

// ---- optional ints ----
impl From<Option<i8>> for MySqlParam {
    fn from(v: Option<i8>) -> Self {
        MySqlParam::OptI8(v)
    }
}
impl From<Option<i16>> for MySqlParam {
    fn from(v: Option<i16>) -> Self {
        MySqlParam::OptI16(v)
    }
}
impl From<Option<i32>> for MySqlParam {
    fn from(v: Option<i32>) -> Self {
        MySqlParam::OptI32(v)
    }
}
impl From<Option<i64>> for MySqlParam {
    fn from(v: Option<i64>) -> Self {
        MySqlParam::OptI64(v)
    }
}
impl From<Option<u8>> for MySqlParam {
    fn from(v: Option<u8>) -> Self {
        MySqlParam::OptU8(v)
    }
}
impl From<Option<u16>> for MySqlParam {
    fn from(v: Option<u16>) -> Self {
        MySqlParam::OptU16(v)
    }
}
impl From<Option<u32>> for MySqlParam {
    fn from(v: Option<u32>) -> Self {
        MySqlParam::OptU32(v)
    }
}
impl From<Option<u64>> for MySqlParam {
    fn from(v: Option<u64>) -> Self {
        MySqlParam::OptU64(v)
    }
}

// ---- optional floats ----
impl From<Option<f32>> for MySqlParam {
    fn from(v: Option<f32>) -> Self {
        MySqlParam::OptF32(v)
    }
}
impl From<Option<f64>> for MySqlParam {
    fn from(v: Option<f64>) -> Self {
        MySqlParam::OptF64(v)
    }
}

// ---- optional bool ----
impl From<Option<bool>> for MySqlParam {
    fn from(v: Option<bool>) -> Self {
        MySqlParam::OptBool(v)
    }
}

// ---- optional strings ----
impl From<Option<String>> for MySqlParam {
    fn from(v: Option<String>) -> Self {
        MySqlParam::OptString(v)
    }
}
impl From<Option<&str>> for MySqlParam {
    fn from(v: Option<&str>) -> Self {
        MySqlParam::OptString(v.map(|s| s.to_owned()))
    }
}

// ---- optional bytes ----
impl From<Option<Vec<u8>>> for MySqlParam {
    fn from(v: Option<Vec<u8>>) -> Self {
        MySqlParam::OptBytes(v)
    }
}

// ---- chrono ----
impl From<NaiveDate> for MySqlParam {
    fn from(v: NaiveDate) -> Self {
        MySqlParam::NaiveDate(v)
    }
}
impl From<NaiveDateTime> for MySqlParam {
    fn from(v: NaiveDateTime) -> Self {
        MySqlParam::NaiveDateTime(v)
    }
}
impl From<DateTime<Utc>> for MySqlParam {
    fn from(v: DateTime<Utc>) -> Self {
        MySqlParam::DateTimeUtc(v)
    }
}
impl From<Option<NaiveDate>> for MySqlParam {
    fn from(v: Option<NaiveDate>) -> Self {
        MySqlParam::OptNaiveDate(v)
    }
}
impl From<Option<NaiveDateTime>> for MySqlParam {
    fn from(v: Option<NaiveDateTime>) -> Self {
        MySqlParam::OptNaiveDateTime(v)
    }
}
impl From<Option<DateTime<Utc>>> for MySqlParam {
    fn from(v: Option<DateTime<Utc>>) -> Self {
        MySqlParam::OptDateTimeUtc(v)
    }
}

// ---- uuid ----
impl From<Uuid> for MySqlParam {
    fn from(v: Uuid) -> Self {
        MySqlParam::Uuid(v)
    }
}
impl From<Option<Uuid>> for MySqlParam {
    fn from(v: Option<Uuid>) -> Self {
        MySqlParam::OptUuid(v)
    }
}


impl From<Decimal> for MySqlParam {
    fn from(v: Decimal) -> Self {
        MySqlParam::Decimal(v)
    }
}

impl From<Option<Decimal>> for MySqlParam {
    fn from(v: Option<Decimal>) -> Self {
        MySqlParam::OptDecimal(v)
    }
}
