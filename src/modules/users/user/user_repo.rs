use anyhow::{Error, Result};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::users::user::user_model::{User, UserRole};
use crate::shared::state::AppState;

#[async_trait]
pub trait UserRepositoryInterface {
    async fn get(&self, user_id: Uuid) -> Result<Option<User>, Error>;

    async fn create(&self, user: User) -> Result<User, Error>;
    
    async fn verify_email(&self, user_id: Uuid) -> Result<bool, Error>;

    async fn update_password(&self, user_id: Uuid, password: String) -> Result<Option<User>, Error>;

    async fn update_name(&self, user_id: Uuid, first_name: String, last_name: String) -> Result<Option<User>, Error>;

    async fn update_base_currency(&self, user_id: Uuid, base_currency: String) -> Result<Option<User>, Error>;

    async fn delete(&self, user_id: Uuid) -> Result<(), Error>;

    async fn get_by_email(&self, email: String) -> Result<Option<User>, Error>;

    async fn get_all(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error>;
}

#[derive(Clone)]
pub struct UserRepository {
    pool: MySqlPool,
}

impl From<&AppState> for UserRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl UserRepositoryInterface for UserRepository {

    async fn get(&self, user_id: Uuid) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id AS "id: _",
                email,
                email_verified AS "email_verified: bool",
                password_hash,
                role AS "role: String",
                first_name,
                last_name,
                base_currency_code,
                created_at,
                updated_at
            FROM users
            WHERE id = ?
            "#,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    async fn create(&self, user: User) -> Result<User, Error> {
        let new_id = Uuid::new_v4();

        // Convert the Enum to a string for MySQL insertion
        let role_str = match user.role {
            UserRole::ADMIN => "ADMIN",
            UserRole::USER => "USER",
        };

        sqlx::query!(
            r#"
            INSERT INTO users
                (id, email, password_hash, role, first_name, last_name, base_currency_code)
            VALUES
                (?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id,
            user.email,
            user.password_hash,
            role_str,
            user.first_name,
            user.last_name,
            user.base_currency_code
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id).await?;
        result.ok_or_else(|| Error::msg("User not found after creation"))
    }

    async fn verify_email(&self, user_id: Uuid) -> Result<bool, Error> {
        sqlx::query!(
            "UPDATE users SET email_verified = true WHERE id = ?",
            user_id
        )
            .execute(&self.pool)
            .await?;
        
        Ok(true)
    }

    async fn update_password(&self, user_id: Uuid, password: String) -> Result<Option<User>, Error> {
        sqlx::query!(
            "UPDATE users SET password_hash = ? WHERE id = ?",
            password,
            user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(user_id).await
    }

    async fn update_name(&self, user_id: Uuid, first_name: String, last_name: String) -> Result<Option<User>, Error> {
        sqlx::query!(
            "UPDATE users SET first_name = ?, last_name = ? WHERE id = ?",
            first_name,
            last_name,
            user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(user_id).await
    }

    async fn update_base_currency(&self, user_id: Uuid, base_currency: String) -> Result<Option<User>, Error> {
        sqlx::query!(
            "UPDATE users SET base_currency_code = ? WHERE id = ?",
            base_currency,
            user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(user_id).await
    }

    async fn delete(&self, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!("DELETE FROM users WHERE id = ?", user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_email(&self, email: String) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id AS "id: _",
                email,
                email_verified AS "email_verified: bool",
                password_hash,
                role AS "role: String",
                first_name,
                last_name,
                base_currency_code,
                created_at,
                updated_at
            FROM users
            WHERE email = ?
            "#,
            email
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_all(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT
                id AS "id: _", 
                email,
                email_verified AS "email_verified: bool",
                password_hash, 
                role AS "role: String",
                first_name, 
                last_name, 
                base_currency_code,
                created_at, 
                updated_at
            FROM users
            LIMIT ? OFFSET ?
            "#,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }
}
