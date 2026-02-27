use axum::{extract::{Path, State}, http::StatusCode, routing::{get}, Json, Router};
use uuid::Uuid;

use crate::modules::transactions::{
    transaction_command::*,
    transaction_dto::*,
    transaction_service::{TransactionService, TransactionInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_transactions).post(post_transaction))
        .route("/{transaction_id}", get(get_transaction).put(put_transaction).delete(delete_transaction))
}


#[utoipa::path(
    get,
    path = "/api/services/transactions",
    responses(
        (status = StatusCode::OK, description = "List of Transactions for current user", body = Vec<TransactionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<TransactionResponse>>, AppError> {
    let command = TransactionListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_by_user(command).await?;
    Ok(Json(transactions))
}


#[utoipa::path(
    post,
    path = "/api/services/transactions",
    responses(
        (status = StatusCode::CREATED, description = "Transaction successfully created", body = TransactionResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn post_transaction(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<TransactionCreateRequest>
) -> Result<Json<TransactionResponse>, AppError> {
    let command = TransactionCreateCommand::new(create_request, auth_user);
    let transaction_service = TransactionService::from(&state);

    let transaction = transaction_service.create(command).await?;
    Ok(Json(transaction))
}


#[utoipa::path(
    get,
    path = "/api/services/transactions/{transaction_id}",
    params(
        ("transaction_id", description = "transaction identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Transaction found successfully", body = TransactionResponse),
        (status = StatusCode::NOT_FOUND, description = "Transaction not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transaction(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(transaction_id): Path<Uuid>,
) -> Result<Json<TransactionResponse>, AppError> {
    let command = TransactionGetCommand::new(transaction_id, auth_user);
    let transaction_service = TransactionService::from(&state);

    let transaction = transaction_service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Transaction {} not found", transaction_id)))?;

    Ok(Json(transaction))
}


#[utoipa::path(
    put,
    path = "/api/services/transactions/{transaction_id}",
    params(
        ("transaction_id", description = "transaction identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Transaction updated successfully", body = TransactionResponse),
        (status = StatusCode::NOT_FOUND, description = "Transaction not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn put_transaction(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(transaction_id): Path<Uuid>,
    Json(update_request): Json<TransactionUpdateRequest>
) -> Result<Json<TransactionResponse>, AppError> {
    let command = TransactionUpdateCommand::new(transaction_id, update_request, auth_user);
    let transaction_service = TransactionService::from(&state);

    let transaction = transaction_service.update(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Transaction {} not found", transaction_id)))?;

    Ok(Json(transaction))
}


#[utoipa::path(
    delete,
    path = "/api/services/transactions/{transaction_id}",
    params(
        ("transaction_id", description = "transaction identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Transaction deleted"),
        (status = StatusCode::NOT_FOUND, description = "Transaction not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn delete_transaction(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(transaction_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let command = TransactionDeleteCommand::new(transaction_id, auth_user);
    let transaction_service = TransactionService::from(&state);

    transaction_service.delete(command).await?;

    Ok(StatusCode::OK)
}
