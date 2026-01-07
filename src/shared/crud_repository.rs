use async_trait::async_trait;
use anyhow::{Error, Result};
use sqlx::mysql::MySqlRow;

use crate::shared::db::mysql::{FromSqlRow, GenericRepository, MySqlParam};
use crate::shared::log::TimePrinter;

#[async_trait]
pub trait CrudRepository<T>: GenericRepository<T>
where
    T: FromSqlRow + Send + Sync,
{
    // ============ CALL PROCEDURE OPERATIONS ============

    async fn call_procedure(
        &self,
        procedure_name: &str,
        params: Vec<MySqlParam>
    ) -> Result<(), Error> {
        let timer = TimePrinter::with_message(&format!(
            "[REPOSITORY] [CALL PROCEDURE] Procedure: {} ",
            procedure_name
        ));

        let mut query = format!("CALL {}(", procedure_name);
        let placeholders = vec!["?"; params.len()].join(", ");
        query.push_str(&placeholders);
        query.push(')');

        let mut sql_query = sqlx::query(&query);
        for param in params {
            sql_query = param.bind(sql_query);
        }

        let entity_row_result = sql_query
            .execute(self.get_pool())
            .await
            .map_err(|e| Error::msg(e.to_string()));

        match entity_row_result {
            Ok(_) => {
                timer.log();
                Ok(())
            },
            Err(e) => {
                timer.error_with_message(format!("Failed to execute procedure: {}", e).as_str());
                Err(Error::msg(e.to_string()))
            }
        }
    }
    
    async fn call_procedure_for_optional(
        &self,
        procedure_name: &str,
        params: Vec<MySqlParam>,
    ) -> Result<Option<T>, Error> {
        let timer = TimePrinter::with_message(&format!(
            "[REPOSITORY] [CALL PROCEDURE] [FOR OPTIONAL] Procedure: {} ",
            procedure_name
        ));

        let mut query = format!("CALL {}(", procedure_name);
        let placeholders = vec!["?"; params.len()].join(", ");
        query.push_str(&placeholders);
        query.push(')');

        let mut sql_query = sqlx::query(&query);
        for param in params {
            sql_query = param.bind(sql_query);
        }

        let entity_row_result= sql_query
            .fetch_optional(self.get_pool())
            .await
            .map_err(|e| Error::msg(e.to_string()));

        self.parse_entity_from_option_result_sql(timer, entity_row_result)
    }
    
    async fn call_procedure_for_one(
        &self,
        procedure_name: &str,
        params: Vec<MySqlParam>,
    ) -> Result<T, Error> {
        let timer = TimePrinter::with_message(&format!(
            "[REPOSITORY] [CALL PROCEDURE] [FOR ONE] Procedure: {} ",
            procedure_name
        ));

        let mut query = format!("CALL {}(", procedure_name);
        let placeholders = vec!["?"; params.len()].join(", ");
        query.push_str(&placeholders);
        query.push(')');

        let mut sql_query = sqlx::query(&query);
        for param in params {
            sql_query = param.bind(sql_query);
        }

        let entity_row_result= sql_query
            .fetch_one(self.get_pool())
            .await
            .map_err(|e| Error::msg(e.to_string()));

        self.parse_entity_from_result_sql(timer, entity_row_result)
    }

    async fn call_procedure_for_list(
        &self,
        procedure_name: &str,
        params: Vec<MySqlParam>,
    ) -> Result<Vec<T>, Error> {
        let timer = TimePrinter::with_message(&format!(
            "[REPOSITORY] [CALL PROCEDURE] [FOR LIST] Procedure: {} ",
            procedure_name
        ));

        let mut query = format!("CALL {}(", procedure_name);
        let placeholders = vec!["?"; params.len()].join(", ");
        query.push_str(&placeholders);
        query.push(')');

        let mut sql_query = sqlx::query(&query);
        for param in params {
            sql_query = param.bind(sql_query);
        }

        let entity_row_result: Result<Vec<MySqlRow>, Error> = sql_query
            .fetch_all(self.get_pool())
            .await
            .map_err(|e| Error::msg(e.to_string()));

        self.parse_entity_from_result_sql_list(timer, entity_row_result)
    }
}

// Auto-implement CrudRepository for all types that implement GenericRepository
impl<T, U> CrudRepository<T> for U
where
    T: FromSqlRow + Send + Sync,
    U: GenericRepository<T> + Send + Sync,
{
}
