use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router};
use crate::modules::users::auth::auth_command::*;
use crate::modules::users::auth::auth_dto::*;
use crate::modules::users::auth::auth_service::{AuthService, AuthServiceInterface};
use crate::shared::response::ApiResponse;
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState
};


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(me))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/forget-password", post(forget_password))
        .route("/reset-password", post(reset_password))
}


#[utoipa::path(
    post,
    path = "/api/services/auth/login",
    responses(
        (status = StatusCode::OK, description = "Login successful", body = ApiResponse<String>),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    security(),
    tag = "Auth"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let command = LoginCommand::from(login_request);
    let auth_service = AuthService::from(&state);

    let response = auth_service.login(command).await;
    match response {
        Ok(response) => {
            match response {
                Some(response) => {
                    Ok(Json(ApiResponse::success(response)))
                },
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    post,
    path = "/api/services/auth/register",
    responses(
        (status = StatusCode::OK, description = "Register successful", body = ApiResponse<String>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    security(),
    tag = "Auth"
)]
pub async fn register(
    State(state): State<AppState>,
    Json(register_request): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let command = RegisterCommand::from(register_request);
    let auth_service = AuthService::from(&state);

    let response = auth_service.register(command).await;
    match response {
        Ok(response) => {
            if response {
                Ok(Json(ApiResponse::success("User registered successfully".to_string())))
            } else {
                Ok(Json(ApiResponse::error("Failed registered user".to_string())))
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    post,
    path = "/api/services/auth/forget-password",
    responses(
        (status = StatusCode::OK, description = "Password Reset successful", body = ApiResponse<String>),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    security(),
    tag = "Auth"
)]
pub async fn forget_password(
    State(state): State<AppState>,
    Json(forget_password_request): Json<ForgotPasswordRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let command = ForgotPasswordCommand::from(forget_password_request);
    let auth_service = AuthService::from(&state);

    let response = auth_service.forgot_password(command).await;
    match response {
        Ok(response) => {
            match response {
                Some(response) => {
                    if response {
                        Ok(Json(ApiResponse::success("Mail reset password send".to_string())))
                    } else {
                        Ok(Json(ApiResponse::error("Failed reset password".to_string())))
                    }
                },
                None => Err(StatusCode::NOT_FOUND)
            }

        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    post,
    path = "/api/services/auth/reset-password",
    responses(
        (status = StatusCode::OK, description = "Password Reset successful", body = ApiResponse<String>),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    security(),
    tag = "Auth"
)]
pub async fn reset_password(
    State(state): State<AppState>,
    Json(reset_password_request): Json<ResetPasswordRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let command = ResetPasswordCommand::from(reset_password_request);
    let auth_service = AuthService::from(&state);

    let response = auth_service.reset_password(command).await;
    match response {
        Ok(response) => {
            match response {
                Some(response) => {
                    if response {
                        Ok(Json(ApiResponse::success("Password successfully change".to_string())))
                    } else {
                        Ok(Json(ApiResponse::error("Failed reset password".to_string())))
                    }
                },
                None => Err(StatusCode::NOT_FOUND)
            }

        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[utoipa::path(
    get,
    path = "/api/services/auth/me",
    responses(
        (status = StatusCode::OK, description = "User me token", body = AuthUser),
        (status = StatusCode::BAD_REQUEST, description = "Invalid token"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Auth"
)]
pub async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<AuthUser>, StatusCode> {
    Ok(Json(auth_user))
}

