use axum::{extract::{Path, State}, http::StatusCode, routing::{get}, Json, Router};
use uuid::Uuid;

use crate::modules::reviews::sessions::{
    review_session_command::*,
    review_session_dto::*,
    review_session_service::{ReviewSessionService, ReviewSessionInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_reviews).post(create_review))
        .route("/{review_session_id}", get(get_review).put(update_review).delete(delete_review))
}


#[utoipa::path(
    get,
    path = "/api/services/reviews/session",
    responses(
        (status = StatusCode::OK, description = "List of Review Sessions for current user", body = Vec<ReviewSessionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Review Session"
)]
pub async fn get_reviews(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<ReviewSessionResponse>>, AppError> {
    let command = ReviewSessionListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let review_service = ReviewSessionService::from(&state);

    let reviews = review_service.get_by_user(command).await?;
    
    Ok(Json(reviews))
}


#[utoipa::path(
    post,
    path = "/api/services/reviews/session",
    responses(
        (status = StatusCode::CREATED, description = "Review Session successfully created", body = ReviewSessionResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Review Session"
)]
pub async fn create_review(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<ReviewSessionCreateRequest>
) -> Result<Json<ReviewSessionResponse>, AppError> {
    let command = ReviewSessionCreateCommand::new(create_request, auth_user);
    let review_service = ReviewSessionService::from(&state);

    let review = review_service.create(command).await?;
    
    Ok(Json(review))
}


#[utoipa::path(
    get,
    path = "/api/services/reviews/session/{review_session_id}",
    params(
        ("review_session_id", description = "review session identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Review Session found successfully", body = ReviewSessionResponse),
        (status = StatusCode::NOT_FOUND, description = "Review Session not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Review Session"
)]
pub async fn get_review(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(review_session_id): Path<Uuid>,
) -> Result<Json<ReviewSessionResponse>, AppError> {
    let command = ReviewSessionGetCommand::new(review_session_id, auth_user);
    let review_service = ReviewSessionService::from(&state);

    let review = review_service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Review Session {} not found", review_session_id)))?;

    Ok(Json(review))
}


#[utoipa::path(
    put,
    path = "/api/services/reviews/session/{review_session_id}",
    params(
        ("review_session_id", description = "review session identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Review Session updated successfully", body = ReviewSessionResponse),
        (status = StatusCode::NOT_FOUND, description = "Review Session not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Review Session"
)]
pub async fn update_review(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(review_session_id): Path<Uuid>,
    Json(update_request): Json<ReviewSessionUpdateRequest>
) -> Result<Json<ReviewSessionResponse>, AppError> {
    let command = ReviewSessionUpdateCommand::new(review_session_id, update_request, auth_user);
    let review_service = ReviewSessionService::from(&state);

    let review = review_service.update(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Review Session {} not found", review_session_id)))?;

    Ok(Json(review))
}


#[utoipa::path(
    delete,
    path = "/api/services/reviews/session/{review_session_id}",
    params(
        ("review_session_id", description = "review session identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Review Session deleted"),
        (status = StatusCode::NOT_FOUND, description = "Review Session not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Review Session"
)]
pub async fn delete_review(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(review_session_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let command = ReviewSessionDeleteCommand::new(review_session_id, auth_user);
    let review_service = ReviewSessionService::from(&state);

    review_service.delete(command).await?;

    Ok(StatusCode::OK)
}
