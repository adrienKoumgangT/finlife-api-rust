use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::files::file_model::{AppFile, FileStatus};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FileResponse {
    pub file_id: Uuid,
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

impl From<AppFile> for FileResponse {
    fn from(file: AppFile) -> Self {
        Self {
            file_id: file.id.unwrap(),
            user_id: file.user_id,
            original_name: file.original_name,
            storage_path: file.storage_path,
            mime_type: file.mime_type,
            size_bytes: file.size_bytes,
            duration_seconds: file.duration_seconds,
            thumbnail_file_id: file.thumbnail_file_id,
            status: file.status,
            created_at: file.created_at,
            updated_at: file.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FileUploadInitResponse {
    pub file: FileResponse,
    pub upload_url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FileDownloadResponse {
    pub download_url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FileCreateRequest {
    pub original_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub duration_seconds: Option<i32>,
    pub thumbnail_file_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FileStatusUpdateRequest {
    pub status: FileStatus,
}
