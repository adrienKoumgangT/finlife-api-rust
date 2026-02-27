use axum::{extract::{Path, State}, http::StatusCode, routing::{get, put}, Json, Router};
use uuid::Uuid;

use crate::modules::categories::{
    category_command::*,
    category_dto::*,
    category_service::{CategoryService, CategoryInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_categories).post(post_category))
        .route("/{category_id}", get(get_category).put(put_category).delete(delete_category))
        .route("/{category_id}/archived", put(put_archived))
}


#[utoipa::path(
    get,
    path = "/api/services/categories",
    responses(
        (status = StatusCode::OK, description = "List of Categories for current user", body = Vec<CategoryResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Category"
)]
pub async fn get_categories(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<CategoryResponse>>, AppError> {
    let command = CategoryListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let category_service = CategoryService::from(&state);

    let categories = category_service.get_by_user(command).await?;
    Ok(Json(categories))
}


#[utoipa::path(
    post,
    path = "/api/services/categories",
    responses(
        (status = StatusCode::CREATED, description = "Category successfully created", body = CategoryResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Category"
)]
pub async fn post_category(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<CategoryCreateRequest>
) -> Result<Json<CategoryResponse>, AppError> {
    let command = CategoryCreateCommand::new(create_request, auth_user);
    let category_service = CategoryService::from(&state);

    let category = category_service.create(command).await?;
    Ok(Json(category))
}


#[utoipa::path(
    get,
    path = "/api/services/categories/{category_id}",
    params(
        ("category_id", description = "category identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Category found successfully", body = CategoryResponse),
        (status = StatusCode::NOT_FOUND, description = "Category not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Category"
)]
pub async fn get_category(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(category_id): Path<Uuid>,
) -> Result<Json<CategoryResponse>, AppError> {
    let command = CategoryGetCommand::new(category_id, auth_user);
    let category_service = CategoryService::from(&state);

    let category = category_service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Category {} not found", category_id)))?;

    Ok(Json(category))
}


#[utoipa::path(
    put,
    path = "/api/services/categories/{category_id}",
    params(
        ("category_id", description = "category identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Category updated successfully", body = CategoryResponse),
        (status = StatusCode::NOT_FOUND, description = "Category not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Category"
)]
pub async fn put_category(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(category_id): Path<Uuid>,
    Json(update_request): Json<CategoryUpdateRequest>
) -> Result<Json<CategoryResponse>, AppError> {
    let command = CategoryUpdateCommand::new(category_id, update_request, auth_user);
    let category_service = CategoryService::from(&state);

    let category = category_service.update(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Category {} not found", category_id)))?;

    Ok(Json(category))
}


#[utoipa::path(
    put,
    path = "/api/services/categories/{category_id}/archived",
    params(
        ("category_id", description = "category identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Category archived status updated", body = CategoryResponse),
        (status = StatusCode::NOT_FOUND, description = "Category not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Category"
)]
pub async fn put_archived(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(category_id): Path<Uuid>,
    Json(update_request): Json<CategoryUpdateArchivedRequest>
) -> Result<Json<CategoryResponse>, AppError> {
    let command = CategoryArchivedCommand::new(category_id, update_request, auth_user);
    let category_service = CategoryService::from(&state);

    let category = category_service.archived(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Category {} not found", category_id)))?;

    Ok(Json(category))
}


#[utoipa::path(
    delete,
    path = "/api/services/categories/{category_id}",
    params(
        ("category_id", description = "category identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Category deleted"),
        (status = StatusCode::NOT_FOUND, description = "Category not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Category"
)]
pub async fn delete_category(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(category_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let command = CategoryDeleteCommand::new(category_id, auth_user);
    let category_service = CategoryService::from(&state);

    category_service.delete(command).await?;

    Ok(StatusCode::OK)
}
