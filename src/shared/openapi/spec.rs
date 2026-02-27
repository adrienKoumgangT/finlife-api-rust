use utoipa::{
    openapi::{
        security::{Http, HttpAuthScheme, SecurityScheme}
        , SecurityRequirement,
    }, Modify,
    OpenApi
};

use crate::modules::{
    accounts::{account_controller, account_dto},
    budgets::{budget_controller, budget_dto},
    categories::{category_controller, category_dto},
    currencies::{currency_controller, currency_dto},
    emails::{email_controller, email_dto},
    files::{file_controller, file_dto},
    goals::{goal_controller, goal_dto},
    investments::{investment_controller, investment_dto},
    locations::{location_controller, location_dto},
    notifications::{notification_controller, notification_dto},
    payees::{payee_controller, payee_dto},
    people::{people_controller, people_dto},
    projects::{
        project_controller, project_dto,
        project_milestone_dto, project_task_dto
    },
    reviews::{
        sessions::{review_session_controller, review_session_dto},
    },
    transactions::{transaction_controller, transaction_dto},
    users::{
        auth::{auth_controller, auth_dto},
        user::{user_controller, user_dto}
    }
};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // or create if None

        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                Http::new(HttpAuthScheme::Bearer),
            ),
        );

        openapi.security = Some(vec![
            SecurityRequirement::new("bearer_auth", Vec::<String>::new())
        ]);
    }
}

#[derive(OpenApi)]
#[openapi(
    info(version = "1.0.0", title = "Fin life API", description = "Fin life API description"),
    servers(
        (url = "http://localhost:8080", description = "Local server"),
    ),
    tags(
        (name = "Account", description = "Account API endpoints"),
        (name = "Auth", description = "Authentication API endpoints"),
        (name = "Budget", description = "Budget API endpoints"),
        (name = "Category", description = "Category API endpoints"),
        (name = "Currency", description = "Currency API endpoints"),
        (name = "Emails", description = "Email API endpoints"),
        (name = "File Registry", description = "File API endpoints"),
        (name = "FX", description = "FX API endpoints"),
        (name = "Goal", description = "Goal API endpoints"),
        (name = "Investment Portfolio", description = "Investment Portfolio API endpoints"),
        (name = "Location", description = "Location API endpoints"),
        (name = "Notification Type", description = "Notification Type API endpoints"),
        (name = "Notification", description = "Notification API endpoints"),
        (name = "Payee", description = "Payee API endpoints"),
        (name = "People", description = "People API endpoints"),
        (name = "Project", description = "Project API endpoints"),
        (name = "Review Session", description = "Review Session API endpoints"),
        (name = "Transaction", description = "Transaction API endpoints"),
        (name = "User", description = "User Manager API endpoints"),
    ),
    paths(
        account_controller::get_accounts, account_controller::post_account,
        account_controller::get_account, account_controller::put_account, account_controller::delete_account,
        account_controller::put_archived,

        auth_controller::me,
        auth_controller::register,
        auth_controller::login, auth_controller::login_alt,
        auth_controller::get_login_logs,
        auth_controller::request_password_reset, auth_controller::confirm_password_reset,
        auth_controller::request_email_verification, auth_controller::confirm_email_verification,

        budget_controller::get_budgets, budget_controller::post_budget,
        budget_controller::get_budget, budget_controller::put_budget, budget_controller::delete_budget,

        budget_controller::get_envelopes, budget_controller::post_envelope,
        budget_controller::get_envelope, budget_controller::put_envelope, budget_controller::delete_envelope,

        category_controller::get_categories, category_controller::post_category,
        category_controller::get_category, category_controller::put_category, category_controller::delete_category,
        category_controller::put_archived,

        currency_controller::get_currencies, currency_controller::post_currency, currency_controller::put_currency,
        currency_controller::get_currency, currency_controller::delete_currency,
    
        currency_controller::get_fx_rates_by_base_code,
        currency_controller::get_fx_rates, currency_controller::post_fx_rate,
        currency_controller::get_fx_rate, currency_controller::put_fx_rate, currency_controller::delete_fx_rate,

        email_controller::get_templates, email_controller::create_template,
        email_controller::update_template,
        email_controller::get_messages, email_controller::create_message,
        email_controller::get_message,
        email_controller::update_message_status,
        email_controller::get_events, email_controller::create_event,

        file_controller::get_files, file_controller::initiate_upload,
        file_controller::get_file, file_controller::delete_file,
        file_controller::get_download_url,
        file_controller::put_file_status,

        goal_controller::get_goals, goal_controller::post_goal,
        goal_controller::get_goal, goal_controller::put_goal,  goal_controller::delete_goal,

        investment_controller::get_portfolios, investment_controller::create_portfolio,
        investment_controller::get_portfolio, investment_controller::update_portfolio, investment_controller::delete_portfolio,

        investment_controller::get_positions, investment_controller::create_position,
        investment_controller::get_position, investment_controller::update_position, investment_controller::delete_position,

        investment_controller::get_trades, investment_controller::create_trade,
        investment_controller::get_trade, investment_controller::update_trade, investment_controller::delete_trade,

        location_controller::get_locations, location_controller::post_location,
        location_controller::get_location, location_controller::put_location, location_controller::delete_location,
        location_controller::put_lat_long,
        location_controller::put_archived,

        notification_controller::list_notification_types, notification_controller::create_notification_type,
        notification_controller::get_notification_type, notification_controller::update_notification_type,

        notification_controller::get_preferences, notification_controller::put_preference,

        notification_controller::get_notifications, notification_controller::get_notification,
        notification_controller::mark_as_read, notification_controller::archive_notification,

        payee_controller::get_payees, payee_controller::post_payee,
        payee_controller::get_payee, payee_controller::put_payee, payee_controller::delete_payee,

        people_controller::get_people, people_controller::post_person, 
        people_controller::get_person, people_controller::put_person, people_controller::delete_person, 
        people_controller::put_archived,

        project_controller::get_projects, project_controller::post_project,
        project_controller::get_project, project_controller::put_project, project_controller::delete_project,

        project_controller::get_project_tasks, project_controller::post_project_task,
        project_controller::get_project_task, project_controller::put_project_task, project_controller::delete_project_task,

        project_controller::get_project_milestones, project_controller::post_project_milestone,
        project_controller::get_project_milestone, project_controller::put_project_milestone, project_controller::delete_project_milestone,

        review_session_controller::get_reviews, review_session_controller::create_review,
        review_session_controller::get_review, review_session_controller::update_review, review_session_controller::delete_review,

        transaction_controller::get_transactions, transaction_controller::post_transaction,
        transaction_controller::get_transaction, transaction_controller::put_transaction, transaction_controller::delete_transaction,

        user_controller::get_users, user_controller::post_user,
        user_controller::get_user, user_controller::put_user, user_controller::delete_user,
        user_controller::put_user_currency,
    ),
    components(
        schemas(
            account_dto::AccountResponse, account_dto::AccountCreateRequest, account_dto::AccountUpdateRequest, account_dto::AccountUpdateArchivedRequest,

            auth_dto::LoginRequest, auth_dto::RegisterRequest, auth_dto::ResetPasswordRequest,
            auth_dto::TokenGenerationResult, auth_dto::PasswordResetRequest, auth_dto::PasswordResetConfirmRequest,
            auth_dto::EmailVerifyRequest, auth_dto::EmailVerifyConfirmRequest,
            auth_dto::LoginLogResponse,

            budget_dto::BudgetResponse, budget_dto::BudgetCreateRequest, budget_dto::BudgetUpdateRequest,

            budget_dto::BudgetEnvelopeResponse, budget_dto::BudgetEnvelopeCreateRequest, budget_dto::BudgetEnvelopeUpdateRequest,

            category_dto::CategoryResponse, category_dto::CategoryCreateRequest, category_dto::CategoryUpdateRequest, category_dto::CategoryUpdateArchivedRequest,

            currency_dto::CurrencyResponse, currency_dto::CurrencyCreateRequest, currency_dto::CurrencyUpdateNameRequest,
            
            currency_dto::FxRateResponse, currency_dto::FxRateCreateRequest, currency_dto::FxRateUpdateRateRequest,

            email_dto::EmailTemplateResponse, email_dto::EmailTemplateCreateRequest, email_dto::EmailTemplateUpdateRequest,
            email_dto::EmailMessageResponse, email_dto::EmailMessageCreateRequest, email_dto::EmailMessageUpdateStatusRequest,
            email_dto::EmailEventResponse, email_dto::EmailEventCreateRequest,

            file_dto::FileResponse,
            file_dto::FileUploadInitResponse, file_dto::FileDownloadResponse,
            file_dto::FileCreateRequest, file_dto::FileStatusUpdateRequest,

            goal_dto::GoalResponse, goal_dto::GoalCreateRequest, goal_dto::GoalUpdateRequest,

            investment_dto::PortfolioResponse, investment_dto::PortfolioCreateRequest, investment_dto::PortfolioUpdateRequest,
            investment_dto::PositionResponse, investment_dto::PositionCreateRequest, investment_dto::PositionUpdateRequest,
            investment_dto::TradeResponse, investment_dto::TradeCreateRequest, investment_dto::TradeUpdateRequest,

            location_dto::LocationResponse,
            location_dto::LocationCreateRequest, location_dto::LocationUpdateRequest, location_dto::LocationUpdateArchivedRequest,
            location_dto::LocationUpdateLatLongRequest,

            notification_dto::NotificationTypeResponse, notification_dto::NotificationTypeCreateRequest, notification_dto::NotificationTypeUpdateRequest,
            notification_dto::NotificationResponse, notification_dto::ListFilter,
            notification_dto::NotificationPreferenceResponse, notification_dto::PreferenceUpdateRequest,

            payee_dto::PayeeResponse, payee_dto::PayeeCreateRequest, payee_dto::PayeeUpdateRequest,

            people_dto::PeopleResponse,
            people_dto::PeopleCreateRequest, people_dto::PeopleUpdateRequest, people_dto::PeopleUpdateArchivedRequest,

            project_dto::ProjectResponse, project_dto::ProjectCreateRequest, project_dto::ProjectUpdateRequest,
            project_milestone_dto::ProjectMilestoneResponse, project_milestone_dto::ProjectMilestoneCreateRequest, project_milestone_dto::ProjectMilestoneUpdateRequest,
            project_task_dto::ProjectTaskResponse, project_task_dto::ProjectTaskCreateRequest, project_task_dto::ProjectTaskUpdateRequest,

            review_session_dto::ReviewSessionResponse, review_session_dto::ReviewSessionCreateRequest, review_session_dto::ReviewSessionUpdateRequest,

            transaction_dto::TransactionResponse, transaction_dto::TransactionCreateRequest, transaction_dto::TransactionUpdateRequest,

            user_dto::UserResponse,
            user_dto::UserCreateRequest, user_dto::UserUpdateNameRequest, user_dto::UserUpdateBaseCurrencyRequest,
        ),
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;
