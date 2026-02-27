use axum::{extract::{Path, State}, http::StatusCode, routing::{get}, Json, Router};
use uuid::Uuid;

use crate::modules::investments::{
    investment_command::*,
    investment_dto::*,
    investment_service::{InvestmentService, InvestmentInterface},
};
use crate::shared::{auth::jwt::AuthUser, state::AppState, errors::AppError};


pub fn routes() -> Router<AppState> {
    Router::new()
        // --- Portfolios ---
        .route("/portfolios", get(get_portfolios).post(create_portfolio))
        .route("/portfolios/{portfolio_id}", get(get_portfolio).put(update_portfolio).delete(delete_portfolio))

        // --- Positions ---
        .route("/portfolios/{portfolio_id}/positions", get(get_positions).post(create_position))
        .route("/portfolios/{portfolio_id}/positions/{position_id}", get(get_position).put(update_position).delete(delete_position))

        // --- Trades ---
        .route("/positions/{position_id}/trades", get(get_trades).post(create_trade))
        .route("/positions/{position_id}/trades/{trade_id}", get(get_trade).put(update_trade).delete(delete_trade))
}



// ==========================================
//                 PORTFOLIOS
// ==========================================

#[utoipa::path(
    get,
    path = "/api/services/investments/portfolios",
    responses(
        (status = StatusCode::OK, body = Vec<PortfolioResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn get_portfolios(
    State(state): State<AppState>,
    auth: AuthUser
) -> Result<Json<Vec<PortfolioResponse>>, AppError> {
    let command = PortfolioListByUserCommand::new(auth.user_id, None, auth);
    let investment_service = InvestmentService::from(&state);

    let portfolios = investment_service.list_portfolios(command).await?;

    Ok(Json(portfolios))
}

#[utoipa::path(
    post,
    path = "/api/services/investments/portfolios",
    responses(
        (status = StatusCode::CREATED, body = PortfolioResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn create_portfolio(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<PortfolioCreateRequest>
) -> Result<Json<PortfolioResponse>, AppError> {
    let command = PortfolioCreateCommand::new(req, auth);
    let investment_service = InvestmentService::from(&state);

    let portfolio = investment_service.create_portfolio(command).await?;

    Ok(Json(portfolio))
}

#[utoipa::path(
    get,
    path = "/api/services/investments/portfolios/{portfolio_id}",
    params(
        ("portfolio_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK, body = PortfolioResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn get_portfolio(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(portfolio_id): Path<Uuid>
) -> Result<Json<PortfolioResponse>, AppError> {
    let command = PortfolioGetCommand { portfolio_id, auth_user: auth };
    let investment_service = InvestmentService::from(&state);

    let portfolio = investment_service.get_portfolio(command).await?
        .ok_or_else(|| AppError::NotFound("Not found".to_string()))?;

    Ok(Json(portfolio))
}

#[utoipa::path(
    put,
    path = "/api/services/investments/portfolios/{portfolio_id}",
    params(
        ("portfolio_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK, body = PortfolioResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn update_portfolio(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(portfolio_id): Path<Uuid>,
    Json(req): Json<PortfolioUpdateRequest>
) -> Result<Json<PortfolioResponse>, AppError> {
    let command = PortfolioUpdateCommand::new(portfolio_id, req, auth);
    let investment_service = InvestmentService::from(&state);

    let portfolio = investment_service.update_portfolio(command).await?
        .ok_or_else(|| AppError::NotFound("Not found".to_string()))?;

    Ok(Json(portfolio))
}

#[utoipa::path(
    delete,
    path = "/api/services/investments/portfolios/{portfolio_id}",
    params(
        ("portfolio_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn delete_portfolio(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(portfolio_id): Path<Uuid>
) -> Result<StatusCode, AppError> {
    let command = PortfolioDeleteCommand { portfolio_id, auth_user: auth };
    let investment_service = InvestmentService::from(&state);

    investment_service.delete_portfolio(command).await?;

    Ok(StatusCode::OK)
}



// ==========================================
//                 POSITIONS
// ==========================================

#[utoipa::path(
    get,
    path = "/api/services/investments/portfolios/{portfolio_id}/positions",
    params(
        ("portfolio_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK, body = Vec<PositionResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn get_positions(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(portfolio_id): Path<Uuid>
) -> Result<Json<Vec<PositionResponse>>, AppError> {
    let command = PositionListByPortfolioCommand { portfolio_id, pagination: None, auth_user: auth };
    let investment_service = InvestmentService::from(&state);

    let positions = investment_service.list_positions(command).await?;

    Ok(Json(positions))
}

#[utoipa::path(
    post,
    path = "/api/services/investments/portfolios/{portfolio_id}/positions",
    params(
        ("portfolio_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::CREATED, body = PositionResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn create_position(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(portfolio_id): Path<Uuid>,
    Json(mut req): Json<PositionCreateRequest>
) -> Result<Json<PositionResponse>, AppError> {
    req.portfolio_id = portfolio_id;

    let command = PositionCreateCommand::new(req, auth);
    let investment_service = InvestmentService::from(&state);

    let position = investment_service.create_position(command).await?;

    Ok(Json(position))
}

#[utoipa::path(
    get,
    path = "/api/services/investments/portfolios/{portfolio_id}/positions/{position_id}",
    params(
        ("portfolio_id", description = "UUID"),
        ("position_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK, body = PositionResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn get_position(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((_port_id, position_id)):
    Path<(Uuid, Uuid)>
) -> Result<Json<PositionResponse>, AppError> {
    let command = PositionGetCommand { position_id, auth_user: auth };
    let investment_service = InvestmentService::from(&state);

    let position = investment_service.get_position(command).await?
        .ok_or_else(|| AppError::NotFound("Not found".to_string()))?;

    Ok(Json(position))
}

#[utoipa::path(
    put,
    path = "/api/services/investments/portfolios/{portfolio_id}/positions/{position_id}",
    params(
        ("portfolio_id", description = "UUID"),
        ("position_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK, body = PositionResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn update_position(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((_port_id, position_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<PositionUpdateRequest>
) -> Result<Json<PositionResponse>, AppError> {
    let command = PositionUpdateCommand::new(position_id, req, auth);
    let investment_service = InvestmentService::from(&state);

    let position = investment_service.update_position(command).await?
        .ok_or_else(|| AppError::NotFound("Not found".to_string()))?;

    Ok(Json(position))
}

#[utoipa::path(
    delete,
    path = "/api/services/investments/portfolios/{portfolio_id}/positions/{position_id}",
    params(
        ("portfolio_id", description = "UUID"),
        ("position_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn delete_position(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((_port_id, position_id)): Path<(Uuid, Uuid)>
) -> Result<StatusCode, AppError> {
    let command = PositionDeleteCommand { position_id, auth_user: auth };
    let investment_service = InvestmentService::from(&state);

    investment_service.delete_position(command).await?;

    Ok(StatusCode::OK)
}



// ==========================================
//                   TRADES
// ==========================================

#[utoipa::path(
    get,
    path = "/api/services/investments/positions/{position_id}/trades",
    params(
        ("position_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK, body = Vec<TradeResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn get_trades(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(position_id): Path<Uuid>
) -> Result<Json<Vec<TradeResponse>>, AppError> {
    let command = TradeListByPositionCommand { position_id, pagination: None, auth_user: auth };
    let investment_service = InvestmentService::from(&state);

    let trades = investment_service.list_trades(command).await?;

    Ok(Json(trades))
}

#[utoipa::path(
    post,
    path = "/api/services/investments/positions/{position_id}/trades",
    params(
        ("position_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::CREATED, body = TradeResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn create_trade(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(position_id): Path<Uuid>,
    Json(mut req): Json<TradeCreateRequest>
) -> Result<Json<TradeResponse>, AppError> {
    req.position_id = position_id;

    let command = TradeCreateCommand::new(req, auth);
    let investment_service = InvestmentService::from(&state);

    let trade = investment_service.create_trade(command).await?;

    Ok(Json(trade))
}

#[utoipa::path(
    get,
    path = "/api/services/investments/positions/{position_id}/trades/{trade_id}",
    params(
        ("position_id", description = "UUID"),
        ("trade_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK, body = TradeResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn get_trade(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((_pos_id, trade_id)): Path<(Uuid, Uuid)>
) -> Result<Json<TradeResponse>, AppError> {
    let command = TradeGetCommand { trade_id, auth_user: auth };
    let investment_service = InvestmentService::from(&state);

    let trade = investment_service.get_trade(command).await?
        .ok_or_else(|| AppError::NotFound("Not found".to_string()))?;

    Ok(Json(trade))
}

#[utoipa::path(
    put,
    path = "/api/services/investments/positions/{position_id}/trades/{trade_id}",
    params(
        ("position_id", description = "UUID"),
        ("trade_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK, body = TradeResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn update_trade(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((_pos_id, trade_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<TradeUpdateRequest>
) -> Result<Json<TradeResponse>, AppError> {
    let command = TradeUpdateCommand::new(trade_id, req, auth);
    let investment_service = InvestmentService::from(&state);

    let res = investment_service.update_trade(command).await?
        .ok_or_else(|| AppError::NotFound("Not found".to_string()))?;

    Ok(Json(res))
}

#[utoipa::path(
    delete,
    path = "/api/services/investments/positions/{position_id}/trades/{trade_id}",
    params(
        ("position_id", description = "UUID"),
        ("trade_id", description = "UUID")
    ),
    responses(
        (status = StatusCode::OK),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Investment Portfolio"
)]
pub async fn delete_trade(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((_pos_id, trade_id)): Path<(Uuid, Uuid)>
) -> Result<StatusCode, AppError> {
    let command = TradeDeleteCommand { trade_id, auth_user: auth };
    let investment_service = InvestmentService::from(&state);

    investment_service.delete_trade(command).await?;

    Ok(StatusCode::OK)
}
