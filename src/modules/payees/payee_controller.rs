use axum::{extract::{Path, State}, http::StatusCode, routing::{get}, Json, Router};
use uuid::Uuid;

use crate::modules::payees::{
    payee_command::*,
    payee_dto::*,
    payee_service::{PayeeService, PayeeInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_payees).post(post_payee))
        .route("/{payee_id}", get(get_payee).put(put_payee).delete(delete_payee))
}


#[utoipa::path(
    get,
    path = "/api/services/payees",
    responses(
        (status = StatusCode::OK, description = "List of Payees for current user", body = Vec<PayeeResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Payee"
)]
pub async fn get_payees(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<PayeeResponse>>, AppError> {
    let command = PayeeListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let payee_service = PayeeService::from(&state);

    let payees = payee_service.get_by_user(command).await?;
    Ok(Json(payees))
}


#[utoipa::path(
    post,
    path = "/api/services/payees",
    responses(
        (status = StatusCode::CREATED, description = "Payee successfully created", body = PayeeResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Payee"
)]
pub async fn post_payee(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<PayeeCreateRequest>
) -> Result<Json<PayeeResponse>, AppError> {
    let command = PayeeCreateCommand::new(create_request, auth_user);
    let payee_service = PayeeService::from(&state);

    let payee = payee_service.create(command).await?;
    Ok(Json(payee))
}


#[utoipa::path(
    get,
    path = "/api/services/payees/{payee_id}",
    params(
        ("payee_id", description = "payee identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Payee found successfully", body = PayeeResponse),
        (status = StatusCode::NOT_FOUND, description = "Payee not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Payee"
)]
pub async fn get_payee(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(payee_id): Path<Uuid>,
) -> Result<Json<PayeeResponse>, AppError> {
    let command = PayeeGetCommand::new(payee_id, auth_user);
    let payee_service = PayeeService::from(&state);

    let payee = payee_service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Payee {} not found", payee_id)))?;

    Ok(Json(payee))
}


#[utoipa::path(
    put,
    path = "/api/services/payees/{payee_id}",
    params(
        ("payee_id", description = "payee identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Payee updated successfully", body = PayeeResponse),
        (status = StatusCode::NOT_FOUND, description = "Payee not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Payee"
)]
pub async fn put_payee(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(payee_id): Path<Uuid>,
    Json(update_request): Json<PayeeUpdateRequest>
) -> Result<Json<PayeeResponse>, AppError> {
    let command = PayeeUpdateCommand::new(payee_id, update_request, auth_user);
    let payee_service = PayeeService::from(&state);

    let payee = payee_service.update(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Payee {} not found", payee_id)))?;

    Ok(Json(payee))
}


#[utoipa::path(
    delete,
    path = "/api/services/payees/{payee_id}",
    params(
        ("payee_id", description = "payee identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Payee deleted"),
        (status = StatusCode::NOT_FOUND, description = "Payee not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Payee"
)]
pub async fn delete_payee(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(payee_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let command = PayeeDeleteCommand::new(payee_id, auth_user);
    let payee_service = PayeeService::from(&state);

    payee_service.delete(command).await?;

    Ok(StatusCode::OK)
}
