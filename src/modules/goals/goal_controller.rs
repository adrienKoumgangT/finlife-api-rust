use axum::{extract::{Path, State}, http::StatusCode, routing::{get}, Json, Router};
use uuid::Uuid;

use crate::modules::goals::{
    goal_command::*,
    goal_dto::*,
    goal_service::{GoalService, GoalInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_goals).post(post_goal))
        .route("/{goal_id}", get(get_goal).put(put_goal).delete(delete_goal))
}


#[utoipa::path(
    get,
    path = "/api/services/goals",
    responses(
        (status = StatusCode::OK, description = "List of Goals for current user", body = Vec<GoalResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Goal"
)]
pub async fn get_goals(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<GoalResponse>>, AppError> {
    let command = GoalListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let goal_service = GoalService::from(&state);

    let goals = goal_service.get_by_user(command).await?;
    
    Ok(Json(goals))
}


#[utoipa::path(
    post,
    path = "/api/services/goals",
    responses(
        (status = StatusCode::CREATED, description = "Goal successfully created", body = GoalResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Goal"
)]
pub async fn post_goal(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<GoalCreateRequest>
) -> Result<Json<GoalResponse>, AppError> {
    let command = GoalCreateCommand::new(create_request, auth_user);
    let goal_service = GoalService::from(&state);

    let goal = goal_service.create(command).await?;
    
    Ok(Json(goal))
}


#[utoipa::path(
    get,
    path = "/api/services/goals/{goal_id}",
    params(
        ("goal_id", description = "goal identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Goal found successfully", body = GoalResponse),
        (status = StatusCode::NOT_FOUND, description = "Goal not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Goal"
)]
pub async fn get_goal(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(goal_id): Path<Uuid>,
) -> Result<Json<GoalResponse>, AppError> {
    let command = GoalGetCommand::new(goal_id, auth_user);
    let goal_service = GoalService::from(&state);

    let goal = goal_service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Goal {} not found", goal_id)))?;

    Ok(Json(goal))
}


#[utoipa::path(
    put,
    path = "/api/services/goals/{goal_id}",
    params(
        ("goal_id", description = "goal identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Goal updated successfully", body = GoalResponse),
        (status = StatusCode::NOT_FOUND, description = "Goal not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Goal"
)]
pub async fn put_goal(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(goal_id): Path<Uuid>,
    Json(update_request): Json<GoalUpdateRequest>
) -> Result<Json<GoalResponse>, AppError> {
    let command = GoalUpdateCommand::new(goal_id, update_request, auth_user);
    let goal_service = GoalService::from(&state);

    let goal = goal_service.update(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Goal {} not found", goal_id)))?;

    Ok(Json(goal))
}


#[utoipa::path(
    delete,
    path = "/api/services/goals/{goal_id}",
    params(
        ("goal_id", description = "goal identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Goal deleted"),
        (status = StatusCode::NOT_FOUND, description = "Goal not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Goal"
)]
pub async fn delete_goal(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(goal_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let command = GoalDeleteCommand::new(goal_id, auth_user);
    let goal_service = GoalService::from(&state);

    goal_service.delete(command).await?;

    Ok(StatusCode::OK)
}
