use anyhow::{Error, Result};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::files::file_model::{AppFile, FileStatus};
use crate::shared::state::AppState;


#[async_trait]
pub trait FileRepositoryInterface {
    
    async fn get(&self, file_id: Uuid, user_id: Uuid) -> Result<Option<AppFile>, Error>;
    
    async fn create(&self, file: AppFile, user_id: Uuid) -> Result<AppFile, Error>;
    
    async fn update_status(&self, file_id: Uuid, status: FileStatus, user_id: Uuid) -> Result<Option<AppFile>, Error>;
    
    async fn delete(&self, file_id: Uuid, user_id: Uuid) -> Result<(), Error>;
    
    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<AppFile>, Error>;
    
}

#[derive(Clone)]
pub struct FileRepository {
    pool: MySqlPool,
}

impl From<&AppState> for FileRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl FileRepositoryInterface for FileRepository {

    async fn get(&self, file_id: Uuid, user_id: Uuid) -> Result<Option<AppFile>, Error> {
        let file = sqlx::query_as!(
            AppFile,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                original_name, storage_path, mime_type, size_bytes,
                duration_seconds, thumbnail_file_id AS "thumbnail_file_id: _",
                status AS "status: String", created_at, updated_at
            FROM files
            WHERE id = ? AND user_id = ? AND status != 'DELETED'
            "#,
            file_id, user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(file)
    }

    async fn create(&self, file: AppFile, user_id: Uuid) -> Result<AppFile, Error> {
        let new_id = Uuid::new_v4();
        let status_str = file.status.as_str();

        sqlx::query!(
            r#"
            INSERT INTO files
                (id, user_id, original_name, storage_path, mime_type, size_bytes, duration_seconds, thumbnail_file_id, status)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id, file.user_id, file.original_name, file.storage_path, file.mime_type,
            file.size_bytes, file.duration_seconds, file.thumbnail_file_id, status_str
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("File not found after creation"))
    }

    async fn update_status(&self, file_id: Uuid, status: FileStatus, user_id: Uuid) -> Result<Option<AppFile>, Error> {
        let status_str = status.as_str();

        sqlx::query!(
            "UPDATE files SET status = ? WHERE id = ? AND user_id = ?",
            status_str, file_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(file_id, user_id).await
    }

    async fn delete(&self, file_id: Uuid, user_id: Uuid) -> Result<(), Error> {
        // Instead of hard deleting, it's often safer to soft delete files
        // so a background cronjob can pick them up and delete them from S3/R2 later.
        sqlx::query!("UPDATE files SET status = 'DELETED' WHERE id = ? AND user_id = ?", file_id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<AppFile>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let files = sqlx::query_as!(
            AppFile,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                original_name, storage_path, mime_type, size_bytes,
                duration_seconds, thumbnail_file_id AS "thumbnail_file_id: _",
                status AS "status: String", created_at, updated_at
            FROM files
            WHERE user_id = ? AND status != 'DELETED'
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id, limit_val, offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(files)
    }
}
