use axum::Router;

use crate::modules::{
    accounts::account_controller,
    budgets::budget_controller,
    categories::category_controller,
    currencies::currency_controller,
    emails::email_controller,
    files::file_controller,
    goals::goal_controller,
    investments::investment_controller,
    locations::location_controller,
    notifications::notification_controller,
    payees::payee_controller,
    people::people_controller,
    projects::project_controller,
    reviews::review_route,
    tags::tag_controller,
    transactions::transaction_controller,
    users::{
        auth::auth_controller,
        user::user_controller
    }
};
use crate::shared::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/accounts", account_controller::routes())
        .nest("/auth", auth_controller::routes())
        .nest("/budgets", budget_controller::routes())
        .nest("/categories", category_controller::routes())
        .nest("/currencies", currency_controller::routes())
        .nest("/emails", email_controller::routes())
        .nest("/files", file_controller::routes())
        .nest("/goals", goal_controller::routes())
        .nest("/investments", investment_controller::routes())
        .nest("/locations", location_controller::routes())
        .nest("/notification", notification_controller::routes())
        .nest("/payees", payee_controller::routes())
        .nest("/people", people_controller::routes())
        .nest("/projects", project_controller::routes())
        .nest("/reviews", review_route::routes())
        .nest("/tags", tag_controller::routes())
        .nest("/transactions", transaction_controller::routes())
        .nest("/users", user_controller::routes())
}
