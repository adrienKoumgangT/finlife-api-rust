use axum::{extract::{Path, State}, http::StatusCode, routing::{get, put}, Json, Router};
use uuid::Uuid;

use crate::modules::files::{
    file_command::*,
    file_dto::*,
    file_service::{FileService, FileInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_files).post(initiate_upload))
        .route("/{file_id}", get(get_file).delete(delete_file))
        .route("/{file_id}/download", get(get_download_url))
        .route("/{file_id}/status", put(put_file_status))
}

#[utoipa::path(
    get, 
    path = "/api/services/files",
    responses(
        (status = StatusCode::OK, description = "List of Files for current user", body = Vec<FileResponse>)
    ),
    tag = "File Registry"
)]
pub async fn get_files(
    State(state): State<AppState>, 
    auth_user: AuthUser
) -> Result<Json<Vec<FileResponse>>, AppError> {
    let command = FileListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let service = FileService::from(&state);
    
    let files = service.get_by_user(command).await?;
    
    Ok(Json(files))
}

#[utoipa::path(
    post, path = "/api/services/files",
    responses((status = StatusCode::CREATED, description = "Initiate file upload", body = FileUploadInitResponse)),
    tag = "File Registry"
)]
pub async fn initiate_upload(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<FileCreateRequest>
) -> Result<Json<FileUploadInitResponse>, AppError> {
    let command = FileCreateCommand::new(create_request, auth_user);
    Ok(Json(FileService::from(&state).create_upload(command).await?))
}

#[utoipa::path(
    get, path = "/api/services/files/{file_id}/download",
    params(("file_id", description = "file identifier in uuid")),
    responses((status = StatusCode::OK, description = "Get secure download URL", body = FileDownloadResponse)),
    tag = "File Registry"
)]
pub async fn get_download_url(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(file_id): Path<Uuid>
) -> Result<Json<FileDownloadResponse>, AppError> {
    let command = FileGetCommand::new(file_id, auth_user);
    Ok(Json(FileService::from(&state).get_download_url(command).await?))
}

#[utoipa::path(
    get, 
    path = "/api/services/files/{file_id}",
    params(
        ("file_id", description = "file identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "File found successfully", body = FileResponse)
    ),
    tag = "File Registry"
)]
pub async fn get_file(
    State(state): State<AppState>, 
    auth_user: AuthUser, 
    Path(file_id): Path<Uuid>
) -> Result<Json<FileResponse>, AppError> {
    let command = FileGetCommand::new(file_id, auth_user);
    let service = FileService::from(&state);
    
    let file = service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("File {} not found", file_id)))?;
    
    Ok(Json(file))
}

#[utoipa::path(
    put, 
    path = "/api/services/files/{file_id}/status",
    params(
        ("file_id", description = "file identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "File status updated successfully", body = FileResponse)
    ),
    tag = "File Registry"
)]
pub async fn put_file_status(
    State(state): State<AppState>, 
    auth_user: AuthUser, 
    Path(file_id): Path<Uuid>, 
    Json(update_request): Json<FileStatusUpdateRequest>
) -> Result<Json<FileResponse>, AppError> {
    let command = FileUpdateStatusCommand::new(file_id, update_request, auth_user);
    let service = FileService::from(&state);
    
    let file = service.update_status(command).await?
        .ok_or_else(|| AppError::NotFound(format!("File {} not found", file_id)))?;
    
    Ok(Json(file))
}

#[utoipa::path(
    delete, 
    path = "/api/services/files/{file_id}",
    params(
        ("file_id", description = "file identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "File deleted")
    ),
    tag = "File Registry"
)]
pub async fn delete_file(
    State(state): State<AppState>, 
    auth_user: AuthUser, 
    Path(file_id): Path<Uuid>
) -> Result<StatusCode, AppError> {
    let command = FileDeleteCommand::new(file_id, auth_user);
    let service = FileService::from(&state);

    service.delete(command).await?;
    
    Ok(StatusCode::OK)
}
