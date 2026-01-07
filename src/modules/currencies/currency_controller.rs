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
        .route("/{currency_code}/fx/rates", get(get_fx_rates_by_base_code))
        .route("/fx/rates", get(get_fx_rates).post(post_fx_rate))
        .route("/fx/rates/{fx_rate_id}", get(get_fx_rate).put(put_fx_rate).delete(delete_fx_rate))
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
    let currency_service = CurrencyService::from(&state);
    
    let response = currency_service.delete_currency(command).await;
    match response {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    get,
    path = "/api/services/currencies/fx/rates",
    responses(
        (status = StatusCode::OK, description = "List of Fx Rates", body = Vec<FxRateResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "FX"
)]
pub async fn get_fx_rates(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<Vec<FxRateResponse>>, StatusCode> {
    let command = FxRateListCommand::new(Some(pagination), auth_user);
    let currency_service = CurrencyService::from(&state);

    let fx_rates = currency_service.list_fx_rates(command).await;
    match fx_rates {
        Ok(fx_rates) => Ok(Json(fx_rates)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    post,
    path = "/api/services/currencies/fx/rates",
    responses(
        (status = StatusCode::OK, description = "Create Fx Rate", body = FxRateResponse),
        (status = StatusCode::NOT_FOUND, description = "Currency not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "FX"
)]
pub async fn post_fx_rate(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(fx_rate_create_request): Json<FxRateCreateRequest>,
) -> Result<Json<FxRateResponse>, StatusCode> {
    let command = FxRateCreateCommand::new(fx_rate_create_request, auth_user);
    let currency_service = CurrencyService::from(&state);

    let fx_rate = currency_service.create_fx_rate(command).await;
    match fx_rate {
        Ok(fx_rate) => {
            match fx_rate {
                Some(fx_rate) => Ok(Json(fx_rate)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    get,
    path = "/api/services/currencies/{currency_code}/fx/rates",
    responses(
        (status = StatusCode::OK, description = "List of Fx Rates by base code"),
        (status = StatusCode::NOT_FOUND, description = "Currency not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "FX"
)]
pub async fn get_fx_rates_by_base_code(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(currency_code): Path<String>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<Vec<FxRateResponse>>, StatusCode> {
    let command = FxRateByBaseCodeCommand::new(currency_code, Some(pagination), auth_user);
    let currency_service = CurrencyService::from(&state);

    let fx_rates = currency_service.list_fx_rates_by_base_code(command).await;
    match fx_rates {
        Ok(fx_rates) => {
            match fx_rates {
                Some(fx_rate) => Ok(Json(fx_rate)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    get,
    path = "/api/services/currencies/fx/rates/{fx_rate_id}",
    params(
        ("fx_rate_id", description = "fx_rate id"),
    ),
    responses(
        (status = StatusCode::OK, description = "fx_rate found successfully", body = CurrencyResponse),
        (status = StatusCode::NOT_FOUND, description = "fx_rate not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "FX"
)]
pub async fn get_fx_rate(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(fx_rate_id): Path<Uuid>,
) -> Result<Json<FxRateResponse>, StatusCode> {
    let command = FxRateGetCommand::new(fx_rate_id, auth_user);
    let currency_service = CurrencyService::from(&state);

    let fx_rate = currency_service.get_fx_rate(command).await;
    match fx_rate {
        Ok(fx_rate) => {
            match fx_rate {
                Some(fx_rate) => Ok(Json(fx_rate)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    put,
    path = "/api/services/currencies/fx/rates/{fx_rate_id}",
    params(
        ("fx_rate_id", description = "fx_rate id"),
    ),
    responses(
        (status = StatusCode::OK, description = "fx_rate rate updated successfully", body = CurrencyResponse),
        (status = StatusCode::NOT_FOUND, description = "fx_rate not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "FX"
)]
pub async fn put_fx_rate(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(fx_rate_id): Path<Uuid>,
    Json(fx_rate_update_rate_request): Json<FxRateUpdateRateRequest>
) -> Result<Json<FxRateResponse>, StatusCode> {
    let command = FxRateUpdateRateCommand::new(fx_rate_id, fx_rate_update_rate_request, auth_user);
    let currency_service = CurrencyService::from(&state);

    let fx_rate = currency_service.update_fx_rate(command).await;
    match fx_rate {
        Ok(fx_rate) => {
            match fx_rate {
                Some(fx_rate) => Ok(Json(fx_rate)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    delete,
    path = "/api/services/currencies/fx/rates/{fx_rate_id}",
    params(
        ("fx_rate_id", description = "fx_rate id"),
    ),
    responses(
        (status = StatusCode::OK, description = "fx_rate rate deleted successfully", body = CurrencyResponse),
        (status = StatusCode::NOT_FOUND, description = "fx_rate not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "FX"
)]
pub async fn delete_fx_rate(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(fx_rate_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let command = FxRateDeleteCommand::new(fx_rate_id, auth_user);
    let currency_service = CurrencyService::from(&state);

    let response = currency_service.delete_fx_rate(command).await;
    match response {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
