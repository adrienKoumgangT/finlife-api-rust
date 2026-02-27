use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router};
use axum::http::HeaderMap;

use crate::modules::users::auth::auth_command::*;
use crate::modules::users::auth::auth_dto::*;
use crate::modules::users::auth::auth_service::{AuthService, AuthServiceInterface};
use crate::shared::response::ApiResponse;
use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState
};
use crate::shared::errors::AppError;


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(me))

        .route("/register", post(register))
        .route("/login", post(login))
        .route("/login-alt", post(login_alt))

        .route("/login/logs", get(get_login_logs))

        .route("/password/reset/request", post(request_password_reset))
        .route("/password/reset/confirm", post(confirm_password_reset))

        .route("/email/verification/request", post(request_email_verification))
        .route("/email/verification/confirm", post(confirm_email_verification))
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
    headers: HeaderMap,
    Json(register_request): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let request_ip = headers
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let command = RegisterCommand::from(register_request);
    let auth_service = AuthService::from(&state);

    let response = auth_service.register(command).await?;

    if response {
        Ok(Json(ApiResponse::success("User registered successfully".to_string())))
    } else {
        Ok(Json(ApiResponse::error("Failed registered user".to_string())))
    }
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
    headers: HeaderMap,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let request_ip = headers
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let command = LoginCommand::new(login_request.email, login_request.password, request_ip, user_agent);
    let auth_service = AuthService::from(&state);

    let response = auth_service.login(command).await?;
    match response {
        Some(response) => {
            Ok(Json(ApiResponse::success(response)))
        },
        None => Err(AppError::InternalError("Login failed".to_string()))
    }
}


#[utoipa::path(
    post,
    path = "/api/services/auth/login-alt",
    responses(
        (status = StatusCode::OK, description = "Login successful", body = ApiResponse<String>),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    security(),
    tag = "Auth"
)]
pub async fn login_alt(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let request_ip = headers
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let command = LoginCommand::new(login_request.email, login_request.password, request_ip, user_agent);
    let auth_service = AuthService::from(&state);

    let response = auth_service.login_alt(command).await?;
    match response {
        Some(response) => {
            Ok(Json(ApiResponse::success(response)))
        },
        None => Err(AppError::InternalError("Login failed".to_string()))
    }
}


#[utoipa::path(
    get,
    path = "/api/services/login/logs",
    responses(
        (status = StatusCode::OK, description = "List of Login Logs for current user", body = Vec<LoginLogResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Auth"
)]
pub async fn get_login_logs(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<LoginLogResponse>>, AppError> {
    let command = LoginLogListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let auth_service = AuthService::from(&state);

    let logs = auth_service.get_login_log_by_user(command).await?;

    Ok(Json(logs))
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
    State(_state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<AuthUser>, AppError> {
    Ok(Json(auth_user))
}





#[utoipa::path(
    post, path = "/api/auth/password/reset/request",
    responses((status = StatusCode::OK, description = "Password reset requested")),
    tag = "Auth"
)]
pub async fn request_password_reset(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<PasswordResetRequest>
) -> Result<StatusCode, AppError> {

    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let request_ip = headers
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let command = CreatePasswordResetCommand::new(req.user_id, request_ip, user_agent);
    let auth_service = AuthService::from(&state);

    auth_service.request_password_reset(command).await?;

    Ok(StatusCode::OK)
}

#[utoipa::path(
    post, path = "/api/auth/password/reset/confirm",
    responses((status = StatusCode::OK, description = "Password successfully reset")),
    tag = "Auth"
)]
pub async fn confirm_password_reset(
    State(state): State<AppState>,
    Json(req): Json<PasswordResetConfirmRequest>
) -> Result<StatusCode, AppError> {
    let command = ConfirmPasswordResetCommand::from(req);
    let auth_service = AuthService::from(&state);

    auth_service.confirm_password_reset(command).await?;

    Ok(StatusCode::OK)
}

#[utoipa::path(
    post, path = "/api/auth/email/verification/request",
    responses((status = StatusCode::OK, description = "Email verification requested")),
    tag = "Auth"
)]
pub async fn request_email_verification(
    State(state): State<AppState>,
    Json(req): Json<EmailVerifyRequest>
) -> Result<StatusCode, AppError> {
    let command = CreateEmailVerifyCommand::from(req);
    let auth_service = AuthService::from(&state);

    auth_service.request_email_verification(command).await?;

    Ok(StatusCode::OK)
}

#[utoipa::path(
    post, path = "/api/auth/email/verification/confirm",
    responses((status = StatusCode::OK, description = "Email successfully verified")),
    tag = "Auth"
)]
pub async fn confirm_email_verification(
    State(state): State<AppState>,
    Json(req): Json<EmailVerifyConfirmRequest>
) -> Result<StatusCode, AppError> {
    let command = ConfirmEmailVerifyCommand::from(req);
    let service = AuthService::from(&state);

    service.confirm_email_verification(command).await?;

    Ok(StatusCode::OK)
}
