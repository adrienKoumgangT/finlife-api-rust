use axum::{extract::{Path, State}, http::StatusCode, routing::{get, put}, Json, Router};
use axum::extract::Query;
use uuid::Uuid;

use crate::modules::locations::{
    location_command::*,
    location_dto::*,
    location_service::{LocationService, LocationServiceInterface}
};
use crate::shared::{
    auth::jwt::AuthUser,
    response::PaginationRequest,
    state::AppState
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_locations).post(post_location))
        .route("/{location_id}", get(get_location).put(put_location).delete(delete_location))
        .route("/{location_id}/archived", put(put_archived))
}


#[utoipa::path(
    get,
    path = "/api/services/locations",
    responses(
        (status = StatusCode::OK, description = "List of location for current user", body = Vec<LocationResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Location"
)]
pub async fn get_locations(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<Vec<LocationResponse>>, StatusCode> {
    let command = LocationListByUserCommand::new(auth_user.user_id.clone(), Some(pagination), auth_user);
    let location_service = LocationService::from(&state);

    let locations = location_service.get_by_user(command).await;
    match locations {
        Ok(locations) => Ok(Json(locations)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    post,
    path = "/api/services/locations",
    responses(
        (status = StatusCode::OK, description = "Person successfully created", body = LocationResponse),
        (status = StatusCode::CREATED, description = "Person successfully created"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Location"
)]
pub async fn post_location(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(location_create_request): Json<LocationCreateRequest>
) -> Result<Json<LocationResponse>, StatusCode> {
    let command = LocationCreateCommand::new(location_create_request, auth_user);
    let location_service = LocationService::from(&state);

    let location = location_service.create(command).await;
    match location {
        Ok(location) => Ok(Json(location)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
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
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Location"
)]
pub async fn get_location(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(location_id): Path<Uuid>
) -> Result<Json<LocationResponse>, StatusCode> {
    let command = LocationGetCommand::new(location_id, auth_user);
    let location_service = LocationService::from(&state);

    let location = location_service.get(command).await;
    match location {
        Ok(location) => {
            match location {
                Some(location) => Ok(Json(location)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
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
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Location"
)]
pub async fn put_location(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(location_id): Path<Uuid>,
    Json(location_update_request): Json<LocationUpdateRequest>
) -> Result<Json<LocationResponse>, StatusCode> {
    let command = LocationUpdateCommand::new(location_id, location_update_request, auth_user);
    let location_service = LocationService::from(&state);

    let location = location_service.update(command).await;
    match location {
        Ok(location) => {
            match location {
                Some(location) => Ok(Json(location)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    put,
    path = "/api/services/locations/{location_id}/archived",
    params(
        ("location_id", description = "location identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Location updated successfully", body = LocationResponse),
        (status = StatusCode::NOT_FOUND, description = "Location not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Location"
)]
pub async fn put_archived(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(location_id): Path<Uuid>,
    Json(location_update_request): Json<LocationUpdateArchivedRequest>
) -> Result<Json<LocationResponse>, StatusCode> {
    let command = LocationArchivedCommand::new(location_id, location_update_request, auth_user);
    let location_service = LocationService::from(&state);

    let location = location_service.archived(command).await;
    match location {
        Ok(location) => {
            match location {
                Some(location) => Ok(Json(location)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    delete,
    path = "/api/services/locations/{location_id}",
    params(
        ("location_id", description = "location identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Location deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "Location not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Location"
)]
pub async fn delete_location(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(location_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let command = LocationDeleteCommand::new(location_id, auth_user);
    let location_service = LocationService::from(&state);

    let location = location_service.delete(command).await;
    match location {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

