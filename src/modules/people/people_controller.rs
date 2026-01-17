use axum::{extract::{Path, State}, http::StatusCode, routing::{get, put}, Json, Router};
use axum::extract::Query;
use uuid::Uuid;

use crate::modules::people::{
    people_command::*,
    people_dto::*,
    people_service::{PeopleService, PeopleInterface},
};
use crate::shared::{
    auth::jwt::AuthUser,
    response::PaginationRequest,
    state::AppState
};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_people).post(post_person))
        .route("/{people_id}", get(get_person).put(put_person).delete(delete_person))
        .route("/{people_id}/archived", put(put_archived))
}


#[utoipa::path(
    get,
    path = "/api/services/people",
    responses(
        (status = StatusCode::OK, description = "List of People for current user", body = Vec<PeopleResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "People"
)]
pub async fn get_people(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<PeopleResponse>>, StatusCode> {
    let command = PeopleListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let people_service = PeopleService::from(&state);
    
    let people = people_service.get_by_user(command).await;
    match people {
        Ok(people) => Ok(Json(people)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    post,
    path = "/api/services/people",
    responses(
        (status = StatusCode::OK, description = "Person successfully created", body = PeopleResponse),
        (status = StatusCode::CREATED, description = "Person already exists"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "People"
)]
pub async fn post_person(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(people_create_request): Json<PeopleCreateRequest>
) -> Result<Json<PeopleResponse>, StatusCode> {
    let command = PeopleCreateCommand::new(people_create_request, auth_user);
    let people_service = PeopleService::from(&state);
    
    let person = people_service.create(command).await;
    match person {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    get,
    path = "/api/services/people/{people_id}",
    params(
        ("people_id", description = "person identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Person found successfully", body = PeopleResponse),
        (status = StatusCode::NOT_FOUND, description = "Person not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "People"
)]
pub async fn get_person(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(people_id): Path<Uuid>,
) -> Result<Json<PeopleResponse>, StatusCode> {
    let command = PeopleGetCommand::new(people_id, auth_user);
    let people_service = PeopleService::from(&state);
    
    let person = people_service.get(command).await;
    match person {
        Ok(person) => {
            match person { 
                Some(person) => Ok(Json(person)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    put,
    path = "/api/services/people/{people_id}",
    params(
        ("people_id", description = "person identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Person updated successfully", body = PeopleResponse),
        (status = StatusCode::NOT_FOUND, description = "Person not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "People"
)]
pub async fn put_person(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(people_id): Path<Uuid>,
    Json(people_update_request): Json<PeopleUpdateRequest>
) -> Result<Json<PeopleResponse>, StatusCode> {
    let command = PeopleUpdateCommand::new(people_id, people_update_request, auth_user);
    let people_service = PeopleService::from(&state);
    
    let person = people_service.update(command).await;
    match person {
        Ok(person) => {
            match person {
                Some(person) => Ok(Json(person)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    put,
    path = "/api/services/people/{people_id}/archived",
    params(
        ("people_id", description = "person identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Person updated successfully", body = PeopleResponse),
        (status = StatusCode::NOT_FOUND, description = "Person not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "People"
)]
pub async fn put_archived(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(people_id): Path<Uuid>,
    Json(people_update_request): Json<PeopleUpdateArchivedRequest>
) -> Result<Json<PeopleResponse>, StatusCode> {
    let command = PeopleArchivedCommand::new(people_id, people_update_request, auth_user);
    let people_service = PeopleService::from(&state);
    
    let person = people_service.archived(command).await;
    match person {
        Ok(person) => {
            match person {
                Some(person) => Ok(Json(person)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    delete,
    path = "/api/services/people/{people_id}",
    params(
        ("people_id", description = "person identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Person deleted"),
        (status = StatusCode::NOT_FOUND, description = "Person not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "People"
)]
pub async fn delete_person(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(people_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let command = PeopleDeleteCommand::new(people_id, auth_user);
    let people_service = PeopleService::from(&state);
    
    let response = people_service.delete(command).await;
    match response {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

