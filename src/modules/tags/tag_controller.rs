use axum::{extract::{Path, State, Query}, http::StatusCode, routing::get, Json, Router};
use uuid::Uuid;

use crate::modules::tags::{
    tag_command::*,
    tag_dto::*,
    tag_service::{TagService, TagInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};
use crate::shared::response::ApiResponse;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_tags).post(post_tag))
        .route("/{tag_id}", get(get_tag).delete(delete_tag))
}


#[utoipa::path(
    get,
    params(TagSearchRequest),
    path = "/api/services/tags",
    responses(
        (status = StatusCode::OK, description = "List of Tags for current user", body = Vec<TagResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Tag"
)]
pub async fn get_tags(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<TagSearchRequest>,
) -> Result<Json<Vec<TagResponse>>, AppError> {
    let command = TagListByUserCommand::new(auth_user.user_id.clone(), query.name, auth_user);
    let service = TagService::from(&state);
    
    let tags = service.get_by_user(command).await?;
    Ok(Json(tags))
}

#[utoipa::path(
    post,
    path = "/api/services/tags",
    responses(
        (status = StatusCode::CREATED, description = "Tag successfully created", body = TagResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Tag"
)]
pub async fn post_tag(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<TagCreateRequest>,
) -> Result<Json<TagResponse>, AppError> {
    let command = TagCreateCommand::new(create_request, auth_user);
    let service = TagService::from(&state);
    
    let tag = service.create(command).await?;
    Ok(Json(tag))
}

#[utoipa::path(
    get,
    path = "/api/services/tags/{tag_id}",
    params(
        ("tag_id", description = "Tag ID"),
    ),
    responses(
        (status = StatusCode::OK, description = "Tag found successfully", body = TagResponse),
        (status = StatusCode::NOT_FOUND, description = "Tag not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Tag"
)]
pub async fn get_tag(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(tag_id): Path<Uuid>,
) -> Result<Json<TagResponse>, AppError> {
    let command = TagGetCommand::new(tag_id, auth_user);
    let service = TagService::from(&state);
    
    let tag = service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Tag {} not found", tag_id)))?;
    
    Ok(Json(tag))
}


#[utoipa::path(
    get,
    path = "/api/services/tags/{tag_id}",
    params(
        ("tag_id", description = "Tag ID"),
    ),
    responses(
        (status = StatusCode::OK, description = "Tag successfully deleted", body = ApiResponse<String>),
        (status = StatusCode::NOT_FOUND, description = "Tag not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Tag"
)]
pub async fn delete_tag(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(tag_id): Path<Uuid>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let command = TagDeleteCommand::new(tag_id, auth_user);
    let service = TagService::from(&state);
    
    service.delete(command).await?;
    
    Ok(Json(ApiResponse::success("Tag deleted successfully".to_string())))
}

