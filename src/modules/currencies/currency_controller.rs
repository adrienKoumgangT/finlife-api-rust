use axum::{extract::{Path, State}, http::StatusCode, routing::{get, put}, Json, Router};
use axum::extract::Query;
use uuid::Uuid;

use crate::modules::currencies::{
    currency_command::*,
    currency_dto::*,
    currency_service::{CurrencyService, CurrencyServiceInterface}
};
use crate::shared::{
    auth::jwt::AuthUser,
    response::PaginationRequest,
    state::AppState
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_currencies).post(post_currency).put(put_currency))
        .route("/{currency_code}", get(get_currency).delete(delete_currency))
}


#[utoipa::path(
    get,
    path = "/api/services/currencies",
    params(
        PaginationRequest
    ),
    responses(
        (status = StatusCode::OK, description = "List of currencies", body = Vec<CurrencyResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Currency"
)]
pub async fn get_currencies(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<Vec<CurrencyResponse>>, StatusCode> {
    let command = CurrencyListCommand::new(pagination, auth_user);
    let currency_service = CurrencyService::from(&state);
    
    let currencies = currency_service.list_currencies(command).await;
    match currencies {
        Ok(currencies) => Ok(Json(currencies)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    post,
    path = "/api/services/currencies",
    responses(
        (status = StatusCode::OK, description = "Currency successfully created", body = CurrencyResponse),
        (status = StatusCode::CREATED, description = "Currency not created"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Currency"
)]
pub async fn post_currency(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(currency_create_request): Json<CurrencyCreateRequest>,
) -> Result<Json<CurrencyResponse>, StatusCode> {
    let command = CurrencyCreateCommand::new(currency_create_request, auth_user);
    let currency_service = CurrencyService::from(&state);
    
    let currency = currency_service.create_currency(command).await;
    match currency {
        Ok(currency) => Ok(Json(currency)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    put,
    path = "/api/services/currencies",
    responses(
        (status = StatusCode::OK, description = "Currency name updated", body = CurrencyResponse),
        (status = StatusCode::NOT_FOUND, description = "Currency not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Currency"
)]
pub async fn put_currency(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(currency_update_request): Json<CurrencyUpdateNameRequest>,
) -> Result<Json<CurrencyResponse>, StatusCode> {
    let command = CurrencyUpdateNameCommand::new(currency_update_request, auth_user);
    let currency_service = CurrencyService::from(&state);

    let currency = currency_service.update_currency_name(command).await;
    match currency {
        Ok(currency) => {
            match currency {
                Some(currency) => Ok(Json(currency)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    get,
    path = "/api/services/currencies/{currency_code}",
    params(
        ("currency_code", description = "Currency code"),
    ),
    responses(
        (status = StatusCode::OK, description = "Currency found successfully", body = CurrencyResponse),
        (status = StatusCode::NOT_FOUND, description = "Currency not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Currency"
)]
pub async fn get_currency(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(currency_code): Path<String>,
) -> Result<Json<CurrencyResponse>, StatusCode> {
    let command = CurrencyGetCommand::new(currency_code, auth_user);
    let currency_service = CurrencyService::from(&state);
    
    let currency = currency_service.get_currency(command).await;
    match currency {
        Ok(currency) => {
            match currency {
                Some(currency) => Ok(Json(currency)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}



#[utoipa::path(
    delete,
    path = "/api/services/currencies/{currency_code}",
    params(
        ("currency_code", description = "Currency code"),
    ),
    responses(
        (status = StatusCode::OK, description = "Currency name deleted"),
        (status = StatusCode::NOT_FOUND, description = "Currency not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Currency"
)]
pub async fn delete_currency(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(currency_code): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let command = CurrencyDeleteCommand::new(currency_code, auth_user);
    let concurrency_service = CurrencyService::from(&state);
    
    let response = concurrency_service.delete_currency(command).await;
    match response {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

