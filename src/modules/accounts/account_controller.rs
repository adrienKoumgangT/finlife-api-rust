use axum::{extract::{Path, State}, http::StatusCode, routing::{get, put}, Json, Router};
use uuid::Uuid;

use crate::modules::accounts::{
    account_command::*,
    account_dto::*,
    account_service::{AccountService, AccountInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_accounts).post(post_account))
        .route("/{account_id}", get(get_account).put(put_account).delete(delete_account))
        .route("/{account_id}/archived", put(put_archived))
}


#[utoipa::path(
    get,
    path = "/api/services/accounts",
    responses(
        (status = StatusCode::OK, description = "List of Accounts for current user", body = Vec<AccountResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Account"
)]
pub async fn get_accounts(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<AccountResponse>>, AppError> {
    let command = AccountListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let account_service = AccountService::from(&state);

    let accounts = account_service.get_by_user(command).await?;
    Ok(Json(accounts))
}


#[utoipa::path(
    post,
    path = "/api/services/accounts",
    responses(
        (status = StatusCode::CREATED, description = "Account successfully created", body = AccountResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Account"
)]
pub async fn post_account(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<AccountCreateRequest>
) -> Result<Json<AccountResponse>, AppError> {
    let command = AccountCreateCommand::new(create_request, auth_user);
    let account_service = AccountService::from(&state);

    let account = account_service.create(command).await?;
    Ok(Json(account))
}


#[utoipa::path(
    get,
    path = "/api/services/accounts/{account_id}",
    params(
        ("account_id", description = "account identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Account found successfully", body = AccountResponse),
        (status = StatusCode::NOT_FOUND, description = "Account not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Account"
)]
pub async fn get_account(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(account_id): Path<Uuid>,
) -> Result<Json<AccountResponse>, AppError> {
    let command = AccountGetCommand::new(account_id, auth_user);
    let account_service = AccountService::from(&state);

    let account = account_service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Account {} not found", account_id)))?;

    Ok(Json(account))
}


#[utoipa::path(
    put,
    path = "/api/services/accounts/{account_id}",
    params(
        ("account_id", description = "account identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Account updated successfully", body = AccountResponse),
        (status = StatusCode::NOT_FOUND, description = "Account not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Account"
)]
pub async fn put_account(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(account_id): Path<Uuid>,
    Json(update_request): Json<AccountUpdateRequest>
) -> Result<Json<AccountResponse>, AppError> {
    let command = AccountUpdateCommand::new(account_id, update_request, auth_user);
    let account_service = AccountService::from(&state);

    let account = account_service.update(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Account {} not found", account_id)))?;

    Ok(Json(account))
}


#[utoipa::path(
    put,
    path = "/api/services/accounts/{account_id}/archived",
    params(
        ("account_id", description = "account identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Account archived status updated", body = AccountResponse),
        (status = StatusCode::NOT_FOUND, description = "Account not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Account"
)]
pub async fn put_archived(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(account_id): Path<Uuid>,
    Json(update_request): Json<AccountUpdateArchivedRequest>
) -> Result<Json<AccountResponse>, AppError> {
    let command = AccountArchivedCommand::new(account_id, update_request, auth_user);
    let account_service = AccountService::from(&state);

    let account = account_service.archived(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Account {} not found", account_id)))?;

    Ok(Json(account))
}


#[utoipa::path(
    delete,
    path = "/api/services/accounts/{account_id}",
    params(
        ("account_id", description = "account identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Account deleted"),
        (status = StatusCode::NOT_FOUND, description = "Account not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Account"
)]
pub async fn delete_account(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(account_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let command = AccountDeleteCommand::new(account_id, auth_user);
    let account_service = AccountService::from(&state);

    account_service.delete(command).await?;

    Ok(StatusCode::OK)
}

