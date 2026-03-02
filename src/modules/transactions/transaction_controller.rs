use axum::{extract::{Path, State, Query}, http::StatusCode, routing::{get}, Json, Router};
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
use crate::shared::response::{PaginatedResponse, PaginationRequest};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_transactions).post(post_transaction))
        .route("/{transaction_id}", get(get_transaction).put(put_transaction).delete(delete_transaction))
        .route("/year/{year}/month/{month}", get(get_transaction_filter))
        .route("/by/account/{account_id}", get(get_transactions_by_account))
        .route("/by/category/{category_id}", get(get_transactions_by_category))
        .route("/by/payee/{payee_id}", get(get_transactions_by_payee))
        .route("/by/person/{person_id}", get(get_transactions_by_person))
        .route("/by/location/{location_id}", get(get_transactions_by_location))
        .route("/by/project/{project_id}", get(get_transactions_by_project))
        .route("/by/goal/{goal_id}", get(get_transactions_by_goal))
        .route("/months/cash-flow", get(get_transactions_months_cash_flow))
        .route("/months/category-expenses", get(get_transactions_months_category_expenses))
}


#[utoipa::path(
    get,
    path = "/api/services/transactions",
    params(PaginationRequest),
    responses(
        (status = StatusCode::OK, description = "List of Transactions for current user", body = PaginatedResponse<TransactionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<TransactionResponse>>, AppError> {
    let command = TransactionListByUserCommand::new(auth_user.user_id.clone(), Some(pagination), auth_user);
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


#[utoipa::path(
    get,
    path = "/api/services/transactions/year/{year}/month/{month}",
    params(
        ("year", description = "transaction occurred_at year in number"),
        ("month", description = "transaction occurred_at month in number"),
    ),
    responses(
        (status = StatusCode::OK, description = "Transaction found successfully", body = Vec<TransactionResponse>),
        (status = StatusCode::NOT_FOUND, description = "Transaction not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transaction_filter(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((year, month)): Path<(u32, u32)>,
) -> Result<Json<Vec<TransactionResponse>>, AppError> {
    let command = TransactionListFilterByUserCommand::new(auth_user.user_id.clone(), year, month, auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_by_user_filter(command).await?;

    Ok(Json(transactions))
}




#[utoipa::path(
    get,
    path = "/api/services/transactions/by/account/{account_id}",
    params(
        ("account_id", description = "account identifier in uuid"),
        PaginationRequest
    ),
    responses(
        (status = StatusCode::OK, description = "List of Transactions for current user", body = PaginatedResponse<TransactionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions_by_account(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(account_id): Path<Uuid>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<TransactionResponse>>, AppError> {
    let command = TransactionListByCommand::new(auth_user.user_id.clone(), account_id, Some(pagination), auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_by_account(command).await?;
    Ok(Json(transactions))
}


#[utoipa::path(
    get,
    path = "/api/services/transactions/by/category/{category_id}",
    params(
        ("category_id", description = "category identifier in uuid"),
        PaginationRequest
    ),
    responses(
        (status = StatusCode::OK, description = "List of Transactions for current user", body = PaginatedResponse<TransactionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions_by_category(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(category_id): Path<Uuid>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<TransactionResponse>>, AppError> {
    let command = TransactionListByCommand::new(auth_user.user_id.clone(), category_id, Some(pagination), auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_by_category(command).await?;
    Ok(Json(transactions))
}


#[utoipa::path(
    get,
    path = "/api/services/transactions/by/payee/{payee_id}",
    params(
        ("payee_id", description = "payee identifier in uuid"),
        PaginationRequest
    ),
    responses(
        (status = StatusCode::OK, description = "List of Transactions for current user", body = PaginatedResponse<TransactionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions_by_payee(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(payee_id): Path<Uuid>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<TransactionResponse>>, AppError> {
    let command = TransactionListByCommand::new(auth_user.user_id.clone(), payee_id, Some(pagination), auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_by_payee(command).await?;
    Ok(Json(transactions))
}


#[utoipa::path(
    get,
    path = "/api/services/transactions/by/person/{person_id}",
    params(
        ("person_id", description = "person identifier in uuid"),
        PaginationRequest
    ),
    responses(
        (status = StatusCode::OK, description = "List of Transactions for current user", body = PaginatedResponse<TransactionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions_by_person(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(person_id): Path<Uuid>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<TransactionResponse>>, AppError> {
    let command = TransactionListByCommand::new(auth_user.user_id.clone(), person_id, Some(pagination), auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_by_person(command).await?;
    Ok(Json(transactions))
}


#[utoipa::path(
    get,
    path = "/api/services/transactions/by/location/{location_id}",
    params(
        ("location_id", description = "location identifier in uuid"),
        PaginationRequest
    ),
    responses(
        (status = StatusCode::OK, description = "List of Transactions for current user", body = PaginatedResponse<TransactionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions_by_location(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(location_id): Path<Uuid>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<TransactionResponse>>, AppError> {
    let command = TransactionListByCommand::new(auth_user.user_id.clone(), location_id, Some(pagination), auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_by_location(command).await?;
    Ok(Json(transactions))
}


#[utoipa::path(
    get,
    path = "/api/services/transactions/by/project/{project_id}",
    params(
        ("project_id", description = "project identifier in uuid"),
        PaginationRequest
    ),
    responses(
        (status = StatusCode::OK, description = "List of Transactions for current user", body = PaginatedResponse<TransactionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions_by_project(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(project_id): Path<Uuid>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<TransactionResponse>>, AppError> {
    let command = TransactionListByCommand::new(auth_user.user_id.clone(), project_id, Some(pagination), auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_by_project(command).await?;
    Ok(Json(transactions))
}


#[utoipa::path(
    get,
    path = "/api/services/transactions/by/goal/{goal_id}",
    params(
        ("goal_id", description = "goal identifier in uuid"),
        PaginationRequest
    ),
    responses(
        (status = StatusCode::OK, description = "List of Transactions for current user", body = PaginatedResponse<TransactionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions_by_goal(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(goal_id): Path<Uuid>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<TransactionResponse>>, AppError> {
    let command = TransactionListByCommand::new(auth_user.user_id.clone(), goal_id, Some(pagination), auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_by_goal(command).await?;
    Ok(Json(transactions))
}




#[utoipa::path(
    get,
    path = "/api/services/transactions/months/cash-flow",
    responses(
        (status = StatusCode::OK, description = "List of Transactions Months cash flow for current user", body = Vec<MonthlyFlowResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions_months_cash_flow(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<MonthlyFlowResponse>>, AppError> {
    let command = TransactionGetStatCommand::new(auth_user.user_id.clone(), auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_12_months_cash_flow(command).await?;
    Ok(Json(transactions))
}


#[utoipa::path(
    get,
    path = "/api/services/transactions/months/category-expenses",
    responses(
        (status = StatusCode::OK, description = "List of Transactions Months category expenses for current user", body = Vec<MonthlyCategoryExpenseResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Transaction"
)]
pub async fn get_transactions_months_category_expenses(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<MonthlyCategoryExpenseResponse>>, AppError> {
    let command = TransactionGetStatCommand::new(auth_user.user_id.clone(), auth_user);
    let transaction_service = TransactionService::from(&state);

    let transactions = transaction_service.get_12_months_category_expenses(command).await?;
    Ok(Json(transactions))
}
