use utoipa::{
    openapi::{
        security::{Http, HttpAuthScheme, SecurityScheme}
        , SecurityRequirement,
    }, Modify,
    OpenApi
};

use crate::modules::{
    currencies::{
        currency_controller, currency_dto
    },
    people::{
        people_controller, people_dto
    },
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
        (name = "Auth", description = "Authentication API endpoints"),
        (name = "Currency", description = "Currency API endpoints"),
        (name = "FX", description = "FX API endpoints"),
        (name = "User", description = "User Manager API endpoints"),
    ),
    paths(
        auth_controller::me,
        auth_controller::register, auth_controller::login,
        auth_controller::forget_password, auth_controller::reset_password,

        currency_controller::get_currencies, currency_controller::post_currency, currency_controller::put_currency,
        currency_controller::get_currency, currency_controller::delete_currency,
    
        currency_controller::get_fx_rates_by_base_code,
        currency_controller::get_fx_rates, currency_controller::post_fx_rate,
        currency_controller::get_fx_rate, currency_controller::put_fx_rate, currency_controller::delete_fx_rate,
    
        people_controller::get_people, people_controller::post_person, 
        people_controller::get_person, people_controller::put_person, people_controller::delete_person, 
        people_controller::put_archived, 

        user_controller::get_users, user_controller::post_user,
        user_controller::get_user, user_controller::put_user, user_controller::delete_user,
        user_controller::put_user_currency,
    ),
    components(
        schemas(
            auth_dto::LoginRequest, auth_dto::RegisterRequest, auth_dto::ResetPasswordRequest,

            currency_dto::CurrencyResponse, currency_dto::CurrencyCreateRequest, currency_dto::CurrencyUpdateNameRequest,
            
            currency_dto::FxRateResponse, currency_dto::FxRateCreateRequest, currency_dto::FxRateUpdateRateRequest,
        
            people_dto::PeopleResponse,
            people_dto::PeopleCreateRequest, people_dto::PeopleUpdateRequest, people_dto::PeopleUpdateArchivedRequest,

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
