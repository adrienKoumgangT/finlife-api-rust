use anyhow::{Error, Result};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::users::auth::auth_model::{PasswordResetToken, EmailVerificationToken, LoginLog};
use crate::shared::state::AppState;

#[async_trait]
pub trait AuthRepositoryInterface {

    async fn create_password_reset(&self, token: PasswordResetToken) -> Result<(), Error>;

    async fn consume_password_reset(&self, token_hash: &[u8]) -> Result<Option<Uuid>, Error>;


    async fn create_email_verification(&self, token: EmailVerificationToken) -> Result<(), Error>;

    async fn consume_email_verification(&self, token_hash: &[u8]) -> Result<Option<Uuid>, Error>;


    async fn create_login_log(&self, log: LoginLog) -> Result<LoginLog, Error>;

    async fn get_login_log_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<LoginLog>, Error>;

}

#[derive(Clone)]
pub struct AuthRepository {
    pool: MySqlPool,
}

impl From<&AppState> for AuthRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl AuthRepositoryInterface for AuthRepository {

    async fn create_password_reset(&self, token: PasswordResetToken) -> Result<(), Error> {
        let new_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO password_reset_tokens
                (id, user_id, token_hash, expires_at, request_ip, user_agent)
            VALUES
                (?, ?, ?, ?, ?, ?)
            "#,
            new_id, token.user_id, token.token_hash, token.expires_at, token.request_ip, token.user_agent
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn consume_password_reset(&self, token_hash: &[u8]) -> Result<Option<Uuid>, Error> {
        struct TokenRecord {
            id: Uuid,
            user_id: Uuid,
        }

        let token_record = sqlx::query_as!(
            TokenRecord,
            r#"
            SELECT id AS "id: _", user_id AS "user_id: _"
            FROM password_reset_tokens
            WHERE token_hash = ? AND used_at IS NULL AND expires_at > NOW()
            "#,
            token_hash
        )
            .fetch_optional(&self.pool)
            .await?;

        if let Some(record) = token_record {
            sqlx::query!(
                "UPDATE password_reset_tokens SET used_at = NOW() WHERE id = ?",
                record.id
            )
                .execute(&self.pool)
                .await?;

            return Ok(Some(record.user_id));
        }

        Ok(None)
    }

    async fn create_email_verification(&self, token: EmailVerificationToken) -> Result<(), Error> {
        let new_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO email_verification_tokens
                (id, user_id, token_hash, expires_at)
            VALUES
                (?, ?, ?, ?)
            "#,
            new_id, token.user_id, token.token_hash, token.expires_at
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn consume_email_verification(&self, token_hash: &[u8]) -> Result<Option<Uuid>, Error> {
        struct TokenRecord {
            id: Uuid,
            user_id: Uuid,
        }

        let token_record = sqlx::query_as!(
            TokenRecord,
            r#"
            SELECT id AS "id: _", user_id AS "user_id: _"
            FROM email_verification_tokens
            WHERE token_hash = ? AND used_at IS NULL AND expires_at > NOW()
            "#,
            token_hash
        )
            .fetch_optional(&self.pool)
            .await?;

        if let Some(record) = token_record {
            sqlx::query!(
                "UPDATE email_verification_tokens SET used_at = NOW() WHERE id = ?",
                record.id
            )
                .execute(&self.pool)
                .await?;

            return Ok(Some(record.user_id));
        }

        Ok(None)
    }



    async fn create_login_log(&self, log: LoginLog) -> Result<LoginLog, Error> {
        let new_id = Uuid::new_v4();
        let status_str = log.status.as_str();

        sqlx::query!(
            r#"
            INSERT INTO user_login_logs
                (id, user_id, login_at, ip_address, user_agent, status, failure_reason)
            VALUES
                (?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id, log.user_id, log.login_at, log.ip_address, log.user_agent, status_str, log.failure_reason
        )
            .execute(&self.pool)
            .await?;

        // Return the created record
        let created_log = sqlx::query_as!(
            LoginLog,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _", login_at,
                ip_address, user_agent, status AS "status: String", failure_reason
            FROM user_login_logs
            WHERE id = ?
            "#,
            new_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(created_log)
    }

    async fn get_login_log_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<LoginLog>, Error> {
        let limit_val = limit.unwrap_or(50) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let logs = sqlx::query_as!(
            LoginLog,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _", login_at,
                ip_address, user_agent, status AS "status: String", failure_reason
            FROM user_login_logs
            WHERE user_id = ?
            ORDER BY login_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(logs)
    }
}
