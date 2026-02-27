use axum::{extract::{Path, State}, http::StatusCode, routing::{get, put}, Json, Router};
use uuid::Uuid;

use crate::modules::locations::{
    location_command::*,
    location_dto::*,
    location_service::{LocationService, LocationInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_locations).post(post_location))
        .route("/{location_id}", get(get_location).put(put_location).delete(delete_location))
        .route("/{location_id}/coordinates", put(put_lat_long))
        .route("/{location_id}/archived", put(put_archived))
}


#[utoipa::path(
    get,
    path = "/api/services/locations",
    responses(
        (status = StatusCode::OK, description = "List of Locations for current user", body = Vec<LocationResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Location"
)]
pub async fn get_locations(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<LocationResponse>>, AppError> {
    let command = LocationListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let location_service = LocationService::from(&state);

    let locations = location_service.get_by_user(command).await?;
    Ok(Json(locations))
}


#[utoipa::path(
    post,
    path = "/api/services/locations",
    responses(
        (status = StatusCode::CREATED, description = "Location successfully created", body = LocationResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Location"
)]
pub async fn post_location(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<LocationCreateRequest>
) -> Result<Json<LocationResponse>, AppError> {
    let command = LocationCreateCommand::new(create_request, auth_user);
    let location_service = LocationService::from(&state);

    let location = location_service.create(command).await?;
    Ok(Json(location))
}


#[utoipa::path(
    get,
    path = "/api/services/locations/{location_id}",
    params(
        ("location_id", description = "location identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Location found successfully", body = LocationResponse),
        (status = StatusCode::NOT_FOUND, description = "Location not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Location"
)]
pub async fn get_location(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(location_id): Path<Uuid>,
) -> Result<Json<LocationResponse>, AppError> {
    let command = LocationGetCommand::new(location_id, auth_user);
    let location_service = LocationService::from(&state);

    let location = location_service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Location {} not found", location_id)))?;

    Ok(Json(location))
}


#[utoipa::path(
    put,
    path = "/api/services/locations/{location_id}",
    params(
        ("location_id", description = "location identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Location updated successfully", body = LocationResponse),
        (status = StatusCode::NOT_FOUND, description = "Location not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Location"
)]
pub async fn put_location(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(location_id): Path<Uuid>,
    Json(update_request): Json<LocationUpdateRequest>
) -> Result<Json<LocationResponse>, AppError> {
    let command = LocationUpdateCommand::new(location_id, update_request, auth_user);
    let location_service = LocationService::from(&state);

    let location = location_service.update(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Location {} not found", location_id)))?;

    Ok(Json(location))
}


#[utoipa::path(
    put,
    path = "/api/services/locations/{location_id}/coordinates",
    params(
        ("location_id", description = "location identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Location coordinates updated successfully", body = LocationResponse),
        (status = StatusCode::NOT_FOUND, description = "Location not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Location"
)]
pub async fn put_lat_long(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(location_id): Path<Uuid>,
    Json(update_request): Json<LocationUpdateLatLongRequest>
) -> Result<Json<LocationResponse>, AppError> {
    let command = LocationUpdateLatLongCommand::new(location_id, update_request, auth_user);
    let location_service = LocationService::from(&state);

    let location = location_service.update_lat_long(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Location {} not found", location_id)))?;

    Ok(Json(location))
}


#[utoipa::path(
    put,
    path = "/api/services/locations/{location_id}/archived",
    params(
        ("location_id", description = "location identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Location archived status updated", body = LocationResponse),
        (status = StatusCode::NOT_FOUND, description = "Location not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Location"
)]
pub async fn put_archived(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(location_id): Path<Uuid>,
    Json(update_request): Json<LocationUpdateArchivedRequest>
) -> Result<Json<LocationResponse>, AppError> {
    let command = LocationArchivedCommand::new(location_id, update_request, auth_user);
    let location_service = LocationService::from(&state);

    let location = location_service.archived(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Location {} not found", location_id)))?;

    Ok(Json(location))
}


#[utoipa::path(
    delete,
    path = "/api/services/locations/{location_id}",
    params(
        ("location_id", description = "location identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Location deleted"),
        (status = StatusCode::NOT_FOUND, description = "Location not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Location"
)]
pub async fn delete_location(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(location_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let command = LocationDeleteCommand::new(location_id, auth_user);
    let location_service = LocationService::from(&state);

    location_service.delete(command).await?;

    Ok(StatusCode::OK)
}

