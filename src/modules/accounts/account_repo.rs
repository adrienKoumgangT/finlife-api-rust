use anyhow::{Error, Result};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::accounts::account_model::{Account, AccountType};
use crate::shared::state::AppState;

#[async_trait]
pub trait AccountRepositoryInterface {

    async fn get(&self, account_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Account>, Error>;

    async fn create(&self, account: Account, user_id: Option<Uuid>) -> Result<Account, Error>;

    async fn update(&self, account_id: Uuid, name: String, account_type: AccountType, institution: Option<String>, user_id: Option<Uuid>) -> Result<Option<Account>, Error>;

    async fn archived(&self, account_id: Uuid, archived: bool, user_id: Option<Uuid>) -> Result<Option<Account>, Error>;

    async fn delete(&self, account_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Account>, Error>;

}

#[derive(Clone)]
pub struct AccountRepository {
    pool: MySqlPool,
}

impl From<&AppState> for AccountRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl AccountRepositoryInterface for AccountRepository {

    async fn get(&self, account_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Account>, Error> {
        let account = sqlx::query_as!(
            Account,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, account_type AS "account_type: String", currency_code, institution,
                archived AS "archived: bool",
                created_at, updated_at
            FROM accounts
            WHERE id = ? AND user_id = ?
            "#,
            account_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(account)
    }

    async fn create(&self, account: Account, user_id: Option<Uuid>) -> Result<Account, Error> {
        let new_id = Uuid::new_v4();
        let type_str = account.account_type.as_str();

        sqlx::query!(
            r#"
            INSERT INTO accounts
                (id, user_id, name, account_type, currency_code, institution, archived)
            VALUES
                (?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id,
            account.user_id,
            account.name,
            type_str,
            account.currency_code,
            account.institution,
            account.archived
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Account not found after creation"))
    }

    async fn update(&self, account_id: Uuid, name: String, account_type: AccountType, institution: Option<String>, user_id: Option<Uuid>) -> Result<Option<Account>, Error> {
        let type_str = account_type.as_str();

        sqlx::query!(
            "UPDATE accounts 
            SET name = ?, account_type = ?, institution = ? 
            WHERE id = ? AND user_id = ?",
            name, type_str, institution, account_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(account_id, user_id).await
    }

    async fn archived(&self, account_id: Uuid, archived: bool, user_id: Option<Uuid>) -> Result<Option<Account>, Error> {
        sqlx::query!(
            "UPDATE accounts SET archived = ? WHERE id = ? AND user_id = ?",
            archived,
            account_id,
            user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(account_id, user_id).await
    }

    async fn delete(&self, account_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!("DELETE FROM accounts WHERE id = ? AND user_id = ?", account_id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Account>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let accounts = sqlx::query_as!(
            Account,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, account_type AS "account_type: String", currency_code, institution,
                archived AS "archived: bool",
                created_at, updated_at
            FROM accounts
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(accounts)
    }
}
