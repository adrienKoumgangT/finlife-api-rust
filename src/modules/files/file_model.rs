use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::files::file_command::FileCreateCommand;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FileStatus {
    Uploading,
    Ready,
    Failed,
    Deleted,
}

impl From<String> for FileStatus {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "READY" => FileStatus::Ready,
            "FAILED" => FileStatus::Failed,
            "DELETED" => FileStatus::Deleted,
            _ => FileStatus::Uploading,
        }
    }
}

impl FileStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileStatus::Uploading => "UPLOADING",
            FileStatus::Ready => "READY",
            FileStatus::Failed => "FAILED",
            FileStatus::Deleted => "DELETED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppFile {
    pub id: Option<Uuid>,
    pub user_id: Uuid,

    pub original_name: String,
    pub storage_path: String,
    pub mime_type: String,
    pub size_bytes: i64,

    pub duration_seconds: Option<i32>,
    pub thumbnail_file_id: Option<Uuid>,

    pub status: FileStatus,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<FileCreateCommand> for AppFile {
    fn from(command: FileCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            original_name: command.original_name,
            storage_path: command.storage_path,
            mime_type: command.mime_type,
            size_bytes: command.size_bytes,
            duration_seconds: command.duration_seconds,
            thumbnail_file_id: command.thumbnail_file_id,
            status: command.status.unwrap_or(FileStatus::Uploading),
            created_at: None,
            updated_at: None,
        }
    }
}
