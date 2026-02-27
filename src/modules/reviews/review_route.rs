use axum::Router;

use crate::modules::reviews::sessions::review_session_controller;
use crate::shared::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/session", review_session_controller::routes())
}
