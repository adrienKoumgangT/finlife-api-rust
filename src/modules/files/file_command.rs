use chrono::{Datelike, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::files::file_dto::*;
use crate::modules::files::file_model::FileStatus;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;


#[derive(Debug, Serialize, Deserialize)]
pub struct FileGetCommand {
    pub file_id: Uuid,
    pub auth_user: AuthUser,
}

impl FileGetCommand {
    pub fn new(file_id: Uuid, auth_user: AuthUser) -> Self {
        Self { file_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCreateCommand {
    pub user_id: Uuid,
    pub original_name: String,
    pub storage_path: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub duration_seconds: Option<i32>,
    pub thumbnail_file_id: Option<Uuid>,
    pub status: Option<FileStatus>,
    pub auth_user: AuthUser,
}

impl FileCreateCommand {
    pub fn new(request: FileCreateRequest, auth_user: AuthUser) -> Self {
        let safe_name = request.original_name.replace(|c: char| !c.is_ascii_alphanumeric() && c != '.', "_");

        let now = Utc::now();
        let year = now.year();
        let month = format!("{:02}", now.month()); // Zero-padded month (e.g., "03")

        let storage_path = format!("{}/{}/{}/{}_{}", auth_user.user_id, year, month, Uuid::new_v4(), safe_name);

        Self {
            user_id: auth_user.user_id,
            original_name: request.original_name,
            storage_path,
            mime_type: request.mime_type,
            size_bytes: request.size_bytes,
            duration_seconds: request.duration_seconds,
            thumbnail_file_id: request.thumbnail_file_id,
            status: Some(FileStatus::Uploading),
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileUpdateStatusCommand {
    pub file_id: Uuid,
    pub status: FileStatus,
    pub auth_user: AuthUser,
}

impl FileUpdateStatusCommand {
    pub fn new(file_id: Uuid, request: FileStatusUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            file_id,
            status: request.status,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDeleteCommand {
    pub file_id: Uuid,
    pub auth_user: AuthUser,
}

impl FileDeleteCommand {
    pub fn new(file_id: Uuid, auth_user: AuthUser) -> Self {
        Self { file_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl FileListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}
