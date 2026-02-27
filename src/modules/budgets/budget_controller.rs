use axum::{extract::{Path, State}, http::StatusCode, routing::{get}, Json, Router};
use uuid::Uuid;

use crate::modules::budgets::{
    budget_command::*,
    budget_dto::*,
    budget_service::{BudgetService, BudgetInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // --- Budgets ---
        .route("/", get(get_budgets).post(post_budget))
        .route("/{budget_id}", get(get_budget).put(put_budget).delete(delete_budget))

        // --- Budget Envelopes ---
        .route("/{budget_id}/envelopes", get(get_envelopes).post(post_envelope))
        .route("/{budget_id}/envelopes/{envelope_id}", get(get_envelope).put(put_envelope).delete(delete_envelope))
}

// ==========================================
//                 BUDGETS
// ==========================================

#[utoipa::path(
    get,
    path = "/api/services/budgets",
    responses(
        (status = StatusCode::OK, description = "List of Budgets for current user", body = Vec<BudgetResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Budget"
)]
pub async fn get_budgets(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<BudgetResponse>>, AppError> {
    let command = BudgetListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let budget_service = BudgetService::from(&state);

    let budgets = budget_service.get_budgets_by_user(command).await?;
    Ok(Json(budgets))
}


#[utoipa::path(
    post,
    path = "/api/services/budgets",
    responses(
        (status = StatusCode::CREATED, description = "Budget successfully created", body = BudgetResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Budget"
)]
pub async fn post_budget(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<BudgetCreateRequest>
) -> Result<Json<BudgetResponse>, AppError> {
    let command = BudgetCreateCommand::new(create_request, auth_user);
    let budget_service = BudgetService::from(&state);

    let budget = budget_service.create_budget(command).await?;
    Ok(Json(budget))
}


#[utoipa::path(
    get,
    path = "/api/services/budgets/{budget_id}",
    params(
        ("budget_id", description = "budget identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Budget found successfully", body = BudgetResponse),
        (status = StatusCode::NOT_FOUND, description = "Budget not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Budget"
)]
pub async fn get_budget(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(budget_id): Path<Uuid>,
) -> Result<Json<BudgetResponse>, AppError> {
    let command = BudgetGetCommand::new(budget_id, auth_user);
    let budget_service = BudgetService::from(&state);

    let budget = budget_service.get_budget(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Budget {} not found", budget_id)))?;

    Ok(Json(budget))
}


#[utoipa::path(
    put,
    path = "/api/services/budgets/{budget_id}",
    params(
        ("budget_id", description = "budget identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Budget updated successfully", body = BudgetResponse),
        (status = StatusCode::NOT_FOUND, description = "Budget not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Budget"
)]
pub async fn put_budget(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(budget_id): Path<Uuid>,
    Json(update_request): Json<BudgetUpdateRequest>
) -> Result<Json<BudgetResponse>, AppError> {
    let command = BudgetUpdateCommand::new(budget_id, update_request, auth_user);
    let budget_service = BudgetService::from(&state);

    let budget = budget_service.update_budget(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Budget {} not found", budget_id)))?;

    Ok(Json(budget))
}


#[utoipa::path(
    delete,
    path = "/api/services/budgets/{budget_id}",
    params(
        ("budget_id", description = "budget identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Budget deleted"),
        (status = StatusCode::NOT_FOUND, description = "Budget not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Budget"
)]
pub async fn delete_budget(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(budget_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let command = BudgetDeleteCommand::new(budget_id, auth_user);
    let budget_service = BudgetService::from(&state);

    budget_service.delete_budget(command).await?;

    Ok(StatusCode::OK)
}


// ==========================================
//             BUDGET ENVELOPES
// ==========================================

#[utoipa::path(
    get,
    path = "/api/services/budgets/{budget_id}/envelopes",
    params(
        ("budget_id", description = "budget identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "List of Envelopes for budget", body = Vec<BudgetEnvelopeResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Budget"
)]
pub async fn get_envelopes(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(budget_id): Path<Uuid>,
) -> Result<Json<Vec<BudgetEnvelopeResponse>>, AppError> {
    let command = BudgetEnvelopeListByBudgetCommand::new(budget_id, None, auth_user);
    let budget_service = BudgetService::from(&state);

    let envelopes = budget_service.get_envelopes_by_budget(command).await?;
    Ok(Json(envelopes))
}


#[utoipa::path(
    post,
    path = "/api/services/budgets/{budget_id}/envelopes",
    params(
        ("budget_id", description = "budget identifier in uuid")
    ),
    responses(
        (status = StatusCode::CREATED, description = "Envelope successfully created", body = BudgetEnvelopeResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Budget"
)]
pub async fn post_envelope(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(budget_id): Path<Uuid>,
    Json(mut create_request): Json<BudgetEnvelopeCreateRequest>
) -> Result<Json<BudgetEnvelopeResponse>, AppError> {
    // Override the budget_id from the path parameter for safety
    create_request.budget_id = budget_id;

    let command = BudgetEnvelopeCreateCommand::new(create_request, auth_user);
    let budget_service = BudgetService::from(&state);

    let envelope = budget_service.create_envelope(command).await?;
    Ok(Json(envelope))
}


#[utoipa::path(
    get,
    path = "/api/services/budgets/{budget_id}/envelopes/{envelope_id}",
    params(
        ("budget_id", description = "budget identifier in uuid"),
        ("envelope_id", description = "envelope identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Envelope found successfully", body = BudgetEnvelopeResponse),
        (status = StatusCode::NOT_FOUND, description = "Envelope not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Budget"
)]
pub async fn get_envelope(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_budget_id, envelope_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<BudgetEnvelopeResponse>, AppError> {
    let command = BudgetEnvelopeGetCommand::new(envelope_id, auth_user);
    let budget_service = BudgetService::from(&state);

    let envelope = budget_service.get_envelope(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Envelope {} not found", envelope_id)))?;

    Ok(Json(envelope))
}


#[utoipa::path(
    put,
    path = "/api/services/budgets/{budget_id}/envelopes/{envelope_id}",
    params(
        ("budget_id", description = "budget identifier in uuid"),
        ("envelope_id", description = "envelope identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Envelope updated successfully", body = BudgetEnvelopeResponse),
        (status = StatusCode::NOT_FOUND, description = "Envelope not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Budget"
)]
pub async fn put_envelope(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_budget_id, envelope_id)): Path<(Uuid, Uuid)>,
    Json(update_request): Json<BudgetEnvelopeUpdateRequest>
) -> Result<Json<BudgetEnvelopeResponse>, AppError> {
    let command = BudgetEnvelopeUpdateCommand::new(envelope_id, update_request, auth_user);
    let budget_service = BudgetService::from(&state);

    let envelope = budget_service.update_envelope(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Envelope {} not found", envelope_id)))?;

    Ok(Json(envelope))
}


#[utoipa::path(
    delete,
    path = "/api/services/budgets/{budget_id}/envelopes/{envelope_id}",
    params(
        ("budget_id", description = "budget identifier in uuid"),
        ("envelope_id", description = "envelope identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Envelope deleted"),
        (status = StatusCode::NOT_FOUND, description = "Envelope not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Budget"
)]
pub async fn delete_envelope(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_budget_id, envelope_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let command = BudgetEnvelopeDeleteCommand::new(envelope_id, auth_user);
    let budget_service = BudgetService::from(&state);

    budget_service.delete_envelope(command).await?;

    Ok(StatusCode::OK)
}
