use axum::{extract::{Path, State}, http::StatusCode, routing::{get, put}, Json, Router};
use axum::extract::Query;
use uuid::Uuid;

use crate::modules::users::user::{
    user_command::*,
    user_dto::*,
    user_service::{UserService, UserServiceInterface}
};
use crate::shared::{
    auth::jwt::AuthUser,
    response::PaginationRequest,
    state::AppState
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users).post(post_user))
        .route("/{user_id}", get(get_user).put(put_user).delete(delete_user))
        .route("/{user_id}/currency", put(put_user_currency))
}


#[utoipa::path(
    get,
    path = "/api/services/users",
    params(
        PaginationRequest
    ),
    responses(
        (status = StatusCode::OK, description = "List of User", body = Vec<UserResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "User"
)]
pub async fn get_users(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let command = UserListCommand::new(pagination, auth_user);
    let user_service = UserService::from(&state);

    let users = user_service.list(command).await;
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    post,
    path = "/api/services/users",
    responses(
        (status = StatusCode::OK, description = "User successfully created", body = UserResponse),
        (status = StatusCode::CREATED, description = "User successfully created", body = UserResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "User"
)]
pub async fn post_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(user_create_request): Json<UserCreateRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let command = UserCreateCommand::new(user_create_request, auth_user);
    let user_service = UserService::from(&state);

    let user = user_service.create(command).await;
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    get,
    path = "/api/services/users/{user_id}",
    params(
        ("user_id", description = "user identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "User found successfully", body = UserResponse),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "User"
)]
pub async fn get_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, StatusCode> {
    let command = UserGetCommand::new(user_id, auth_user);
    let user_service = UserService::from(&state);

    let user = user_service.get(command).await;
    match user {
        Ok(user) => {
            match user {
                Some(user) => Ok(Json(user)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    put,
    path = "/api/services/users/{user_id}",
    params(
        ("user_id", description = "user identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "User updated successfully", body = UserResponse),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "User"
)]
pub async fn put_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<Uuid>,
    Json(user_update_request): Json<UserUpdateNameRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let command = UserUpdateNameCommand::new(user_id, user_update_request, auth_user);
    let user_service = UserService::from(&state);

    let user = user_service.update_name(command).await;
    match user {
        Ok(user) => {
            match user {
                Some(user) => Ok(Json(user)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    put,
    path = "/api/services/users/{user_id}/currency",
    params(
        ("user_id", description = "user identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "User updated successfully", body = UserResponse),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "User"
)]
pub async fn put_user_currency(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<Uuid>,
    Json(user_update_request): Json<UserUpdateBaseCurrencyRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let command = UserUpdateBaseCurrencyCommand::new(user_id, user_update_request, auth_user);
    let user_service = UserService::from(&state);

    let user = user_service.update_base_currency(command).await;
    match user {
        Ok(user) => {
            match user {
                Some(user) => Ok(Json(user)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    delete,
    path = "/api/services/users/{user_id}",
    params(
        ("user_id", description = "user identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "User deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "User"
)]
pub async fn delete_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let command = UserDeleteCommand::new(user_id, auth_user);
    let user_service = UserService::from(&state);

    let response = user_service.delete(command).await;
    match response {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
