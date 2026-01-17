use axum::Router;

use crate::modules::{
    currencies::currency_controller,
    locations::location_controller,
    people::people_controller,
    users::user::user_controller
};
use crate::shared::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/currencies", currency_controller::routes())
        .nest("locations", location_controller::routes())
        .nest("/people", people_controller::routes())
        .nest("/users", user_controller::routes())
}
