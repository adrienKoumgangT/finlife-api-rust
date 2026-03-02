use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::transactions::transaction_model::{MonthlyCategoryExpense, MonthlyFlow, Transaction, TransactionStatus};
use crate::shared::model::CountRow;
use crate::shared::state::AppState;


#[async_trait]
pub trait TransactionRepositoryInterface {

    async fn get(&self, transaction_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Transaction>, Error>;

    async fn create(&self, transaction: Transaction, user_id: Option<Uuid>) -> Result<Transaction, Error>;

    #[allow(clippy::too_many_arguments)]
    async fn update(&self, transaction_id: Uuid, account_id: Uuid, occurred_at: NaiveDateTime, amount_minor: i64, currency_code: String, base_amount_minor: i64, base_currency_code: String, fx_rate_id: Option<Uuid>, category_id: Option<Uuid>, payee_id: Option<Uuid>, person_id: Option<Uuid>, location_id: Option<Uuid>, note: Option<String>, project_id: Option<Uuid>, goal_id: Option<Uuid>, status: TransactionStatus, user_id: Option<Uuid>) -> Result<Option<Transaction>, Error>;

    async fn delete(&self, transaction_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error>;

    async fn count_by_user(&self, user_id: Uuid) -> Result<i64, Error>;

    async fn get_by_user_filter(&self, user_id: Uuid, year: Option<u32>, month: Option<u32>) -> Result<Vec<Transaction>, Error>;

    async fn get_by_account(&self, user_id: Uuid, account_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error>;

    async fn count_by_account(&self, user_id: Uuid, account_id: Uuid) -> Result<i64, Error>;

    async fn get_by_category(&self, user_id: Uuid, category_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error>;

    async fn count_by_category(&self, user_id: Uuid, category_id: Uuid) -> Result<i64, Error>;

    async fn get_by_payee(&self, user_id: Uuid, payee_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error>;

    async fn count_by_payee(&self, user_id: Uuid, payee_id: Uuid) -> Result<i64, Error>;

    async fn get_by_person(&self, user_id: Uuid, person_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error>;

    async fn count_by_person(&self, user_id: Uuid, person_id: Uuid) -> Result<i64, Error>;

    async fn get_by_location(&self, user_id: Uuid, location_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error>;

    async fn count_by_location(&self, user_id: Uuid, location_id: Uuid) -> Result<i64, Error>;

    async fn get_by_project(&self, user_id: Uuid, project_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error>;

    async fn count_by_project(&self, user_id: Uuid, project_id: Uuid) -> Result<i64, Error>;

    async fn get_by_goal(&self, user_id: Uuid, goal_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error>;

    async fn count_by_goal(&self, user_id: Uuid, goal_id: Uuid) -> Result<i64, Error>;


    async fn get_12_months_cash_flow(&self, user_id: Uuid) -> Result<Vec<MonthlyFlow>, Error>;

    async fn get_12_months_category_expenses(&self, user_id: Uuid) -> Result<Vec<MonthlyCategoryExpense>, Error>;

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

    async fn count_by_user(&self, user_id: Uuid) -> Result<i64, Error> {
        let count = sqlx::query_as!(
            CountRow,
            r#"SELECT IFNULL(COUNT(*), 0) as n FROM transactions WHERE user_id = ?"#,
            user_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(count.n)
    }

    async fn get_by_user_filter(&self, user_id: Uuid, year: Option<u32>, month: Option<u32>) -> Result<Vec<Transaction>, Error> {
        let (start_date, end_date) = match year {
            Some(y) => {
                let m = month.unwrap_or(1);

                let start = NaiveDate::from_ymd_opt(y as i32, m, 1)
                    .and_then(|d| d.and_hms_opt(0, 0, 0))
                    .unwrap_or_else(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap());

                let (next_y, next_m) = if month.is_some() {
                    if m == 12 { (y + 1, 1) } else { (y, m + 1) } // Next month
                } else {
                    (y + 1, 1) // Next year
                };

                let end = NaiveDate::from_ymd_opt(next_y as i32, next_m, 1)
                    .and_then(|d| d.and_hms_opt(0, 0, 0))
                    .unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap());

                (start, end)
            }
            None => {
                let start = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
                let end = NaiveDate::from_ymd_opt(9999, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
                (start, end)
            }
        };

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
          AND occurred_at >= ?
          AND occurred_at < ?
        ORDER BY occurred_at DESC, created_at DESC
        "#,
        user_id,
        start_date,
        end_date
    )
            .fetch_all(&self.pool)
            .await?;

        Ok(transactions)
    }

    async fn get_by_account(&self, user_id: Uuid, account_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error> {
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
            WHERE user_id = ? AND account_id = ?
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            account_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(transactions)
    }

    async fn count_by_account(&self, user_id: Uuid, account_id: Uuid) -> Result<i64, Error> {
        let count = sqlx::query_as!(
            CountRow,
            r#"SELECT IFNULL(COUNT(*), 0) as n FROM transactions WHERE user_id = ? AND account_id = ?"#,
            user_id, account_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(count.n)
    }

    async fn get_by_category(&self, user_id: Uuid, category_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error> {
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
            WHERE user_id = ? AND category_id = ?
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            category_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(transactions)
    }

    async fn count_by_category(&self, user_id: Uuid, category_id: Uuid) -> Result<i64, Error> {
        let count = sqlx::query_as!(
            CountRow,
            r#"SELECT IFNULL(COUNT(*), 0) as n FROM transactions WHERE user_id = ? AND category_id = ?"#,
            user_id, category_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(count.n)
    }

    async fn get_by_payee(&self, user_id: Uuid, payee_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error> {
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
            WHERE user_id = ? AND payee_id = ?
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            payee_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(transactions)
    }

    async fn count_by_payee(&self, user_id: Uuid, payee_id: Uuid) -> Result<i64, Error> {
        let count = sqlx::query_as!(
            CountRow,
            r#"SELECT IFNULL(COUNT(*), 0) as n FROM transactions WHERE user_id = ? AND payee_id = ?"#,
            user_id, payee_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(count.n)
    }

    async fn get_by_person(&self, user_id: Uuid, person_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error> {
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
            WHERE user_id = ? AND person_id = ?
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            person_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(transactions)
    }

    async fn count_by_person(&self, user_id: Uuid, person_id: Uuid) -> Result<i64, Error> {
        let count = sqlx::query_as!(
            CountRow,
            r#"SELECT IFNULL(COUNT(*), 0) as n FROM transactions WHERE user_id = ? AND person_id = ?"#,
            user_id, person_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(count.n)
    }

    async fn get_by_location(&self, user_id: Uuid, location_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error> {
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
            WHERE user_id = ? AND location_id = ?
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            location_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(transactions)
    }

    async fn count_by_location(&self, user_id: Uuid, location_id: Uuid) -> Result<i64, Error> {
        let count = sqlx::query_as!(
            CountRow,
            r#"SELECT IFNULL(COUNT(*), 0) as n FROM transactions WHERE user_id = ? AND location_id = ?"#,
            user_id, location_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(count.n)
    }

    async fn get_by_project(&self, user_id: Uuid, project_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error> {
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
            WHERE user_id = ? AND project_id = ?
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            project_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(transactions)
    }

    async fn count_by_project(&self, user_id: Uuid, project_id: Uuid) -> Result<i64, Error> {
        let count = sqlx::query_as!(
            CountRow,
            r#"SELECT IFNULL(COUNT(*), 0) as n FROM transactions WHERE user_id = ? AND project_id = ?"#,
            user_id, project_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(count.n)
    }

    async fn get_by_goal(&self, user_id: Uuid, goal_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Transaction>, Error> {
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
            WHERE user_id = ? AND goal_id = ?
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            goal_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(transactions)
    }

    async fn count_by_goal(&self, user_id: Uuid, goal_id: Uuid) -> Result<i64, Error> {
        let count = sqlx::query_as!(
            CountRow,
            r#"SELECT IFNULL(COUNT(*), 0) as n FROM transactions WHERE user_id = ? AND goal_id = ?"#,
            user_id, goal_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(count.n)
    }

    async fn get_12_months_cash_flow(&self, user_id: Uuid) -> Result<Vec<MonthlyFlow>, Error> {
        let records = sqlx::query_as!(
            MonthlyFlow,
            r#"
            SELECT
                DATE_FORMAT(occurred_at, '%Y-%m') as month,
                CAST(COALESCE(SUM(CASE WHEN base_amount_minor > 0 THEN base_amount_minor ELSE 0 END), 0) AS SIGNED) as total_income,
                CAST(COALESCE(SUM(CASE WHEN base_amount_minor < 0 THEN ABS(base_amount_minor) ELSE 0 END), 0) AS SIGNED) as total_expense
            FROM transactions
            WHERE user_id = ? AND occurred_at >= DATE_FORMAT(DATE_SUB(CURDATE(), INTERVAL 11 MONTH), '%Y-%m-01')
            GROUP BY DATE_FORMAT(occurred_at, '%Y-%m')
            ORDER BY month ASC
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(records)
    }

    async fn get_12_months_category_expenses(&self, user_id: Uuid) -> Result<Vec<MonthlyCategoryExpense>, Error> {
        let records = sqlx::query_as!(
            MonthlyCategoryExpense,
            r#"
            SELECT
                DATE_FORMAT(occurred_at, '%Y-%m') as month,
                category_id AS "category_id: _",
                CAST(COALESCE(SUM(ABS(base_amount_minor)), 0) AS SIGNED) as total_amount
            FROM transactions
            WHERE user_id = ? AND base_amount_minor < 0
              AND occurred_at >= DATE_FORMAT(DATE_SUB(CURDATE(), INTERVAL 11 MONTH), '%Y-%m-01')
            GROUP BY DATE_FORMAT(occurred_at, '%Y-%m'), category_id
            ORDER BY month ASC, total_amount DESC
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(records)
    }
}
