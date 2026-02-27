use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::transactions::transaction_model::{Transaction, TransactionStatus};
use crate::shared::state::AppState;


#[async_trait]
pub trait TransactionRepositoryInterface {

    async fn get(&self, transaction_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Transaction>, Error>;

    async fn create(&self, transaction: Transaction, user_id: Option<Uuid>) -> Result<Transaction, Error>;

    #[allow(clippy::too_many_arguments)]
    async fn update(&self, transaction_id: Uuid, account_id: Uuid, occurred_at: NaiveDateTime, amount_minor: i64, currency_code: String, base_amount_minor: i64, base_currency_code: String, fx_rate_id: Option<Uuid>, category_id: Option<Uuid>, payee_id: Option<Uuid>, person_id: Option<Uuid>, location_id: Option<Uuid>, note: Option<String>, project_id: Option<Uuid>, goal_id: Option<Uuid>, status: TransactionStatus, user_id: Option<Uuid>) -> Result<Option<Transaction>, Error>;

    async fn delete(&self, transaction_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error>;

}

#[derive(Clone)]
pub struct TransactionRepository {
    pool: MySqlPool,
}

impl From<&AppState> for TransactionRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl TransactionRepositoryInterface for TransactionRepository {

    async fn get(&self, transaction_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Transaction>, Error> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _", account_id AS "account_id: _",
                occurred_at, amount_minor, currency_code, base_amount_minor, base_currency_code,
                fx_rate_id AS "fx_rate_id: _", category_id AS "category_id: _", payee_id AS "payee_id: _",
                person_id AS "person_id: _", location_id AS "location_id: _", note,
                project_id AS "project_id: _", goal_id AS "goal_id: _",
                status AS "status: String", created_at, updated_at
            FROM transactions
            WHERE id = ? AND user_id = ?
            "#,
            transaction_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(transaction)
    }

    async fn create(&self, tx: Transaction, user_id: Option<Uuid>) -> Result<Transaction, Error> {
        let new_id = Uuid::new_v4();
        let status_str = tx.status.as_str();

        sqlx::query!(
            r#"
            INSERT INTO transactions
                (id, user_id, account_id, occurred_at, amount_minor, currency_code, base_amount_minor, base_currency_code, fx_rate_id, category_id, payee_id, person_id, location_id, note, project_id, goal_id, status)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id, tx.user_id, tx.account_id, tx.occurred_at, tx.amount_minor, tx.currency_code, tx.base_amount_minor, tx.base_currency_code, tx.fx_rate_id, tx.category_id, tx.payee_id, tx.person_id, tx.location_id, tx.note, tx.project_id, tx.goal_id, status_str
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Transaction not found after creation"))
    }

    async fn update(
        &self,
        transaction_id: Uuid,
        account_id: Uuid,
        occurred_at: NaiveDateTime,
        amount_minor: i64,
        currency_code: String,
        base_amount_minor: i64,
        base_currency_code: String,
        fx_rate_id: Option<Uuid>,
        category_id: Option<Uuid>,
        payee_id: Option<Uuid>,
        person_id: Option<Uuid>,
        location_id: Option<Uuid>,
        note: Option<String>,
        project_id: Option<Uuid>,
        goal_id: Option<Uuid>,
        status: TransactionStatus,
        user_id: Option<Uuid>
    ) -> Result<Option<Transaction>, Error> {
        let status_str = status.as_str();

        sqlx::query!(
            r#"
            UPDATE transactions SET
                account_id = ?, occurred_at = ?, amount_minor = ?, currency_code = ?, base_amount_minor = ?, base_currency_code = ?,
                fx_rate_id = ?, category_id = ?, payee_id = ?, person_id = ?, location_id = ?, note = ?, project_id = ?, goal_id = ?, status = ?
            WHERE id = ? AND user_id = ?
            "#,
            account_id, occurred_at, amount_minor, currency_code, base_amount_minor, base_currency_code,
            fx_rate_id, category_id, payee_id, person_id, location_id, note, project_id, goal_id, status_str,
            transaction_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(transaction_id, user_id).await
    }

    async fn delete(&self, transaction_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!("DELETE FROM transactions WHERE id = ? AND user_id = ?", transaction_id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _", account_id AS "account_id: _",
                occurred_at, amount_minor, currency_code, base_amount_minor, base_currency_code,
                fx_rate_id AS "fx_rate_id: _", category_id AS "category_id: _", payee_id AS "payee_id: _",
                person_id AS "person_id: _", location_id AS "location_id: _", note,
                project_id AS "project_id: _", goal_id AS "goal_id: _",
                status AS "status: String", created_at, updated_at
            FROM transactions
            WHERE user_id = ?
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(transactions)
    }
}
