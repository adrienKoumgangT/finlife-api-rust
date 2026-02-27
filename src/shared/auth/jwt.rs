use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, HeaderValue},
};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde::__private228::de::IdentifierDeserializer;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::users::user::user_model::{User, UserRole};
use crate::shared::errors::AppError;
use crate::shared::state::AppState;

#[derive(Debug, Clone)]
pub struct JwtVerifier {
    decoding_key: DecodingKey,
    encoding_key: EncodingKey,
    validation: Validation,
    issuer: String,
    audience: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id UUID string
    pub role: UserRole,
    pub iss: String,
    pub aud: String,
    pub exp: usize,
}

impl JwtVerifier {
    pub fn new(public_pem: &str, private_pem: &str, issuer: &str, audience: &str) -> anyhow::Result<Self> {
        let decoding_key = DecodingKey::from_rsa_pem(public_pem.as_bytes())?;
        let encoding_key = EncodingKey::from_rsa_pem(private_pem.as_bytes())?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;
        validation.set_issuer(&[issuer]);
        validation.set_audience(&[audience]);

        Ok(Self {
            decoding_key,
            encoding_key,
            validation,
            issuer: issuer.to_string(),
            audience: audience.to_string(),
        })
    }

    pub fn generate_token(&self, user: AuthUser) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = (now + Duration::try_hours(24).unwrap()).timestamp() as usize;

        let claims = Claims {
            sub: user.user_id.to_string(),
            role: user.role,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            exp,
        };

        // Sign the token using the private key
        encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &self.encoding_key
        ).map_err(|_| AppError::Unauthorized("Error during token generation".to_string())) // Or specific "TokenCreationError"
    }

    pub fn verify(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &self.decoding_key,
            &self.validation,
        ).map_err(|_| AppError::Unauthorized(format!("Invalid token: {}", token)))?;

        let c = token_data.claims;
        if c.iss != self.issuer || c.aud != self.audience {
            return Err(AppError::Unauthorized(format!(
                "Invalid issuer or audience: {} != {}",
                c.iss, self.issuer
            )));
        }
        Ok(c)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub role: UserRole
}

impl From<User> for AuthUser {
    fn from(user: User) -> Self {
        Self { user_id: user.id.unwrap(), role: user.role }
    }
}

impl From<&User> for AuthUser {
    fn from(user: &User) -> Self {
        Self { user_id: user.id.clone().unwrap(), role: user.role.clone() }
    }
}

fn bearer_token(auth: &HeaderValue) -> Option<&str> {
    auth.to_str().ok()?.strip_prefix("Bearer ")
}


impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .ok_or(AppError::Unauthorized("Token not found".to_string()))?;

        let token = bearer_token(auth).ok_or(AppError::Unauthorized("Invalid token format (should be Bearer <token>)".to_string()))?;

        let claims = state.jwt.verify(token)?;
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized("Invalid user_id format (should be UUID string)".to_string()))?;

        Ok(AuthUser { user_id, role: claims.role })
    }
}


pub fn require_admin(user: &AuthUser) -> Result<(), AppError> {
    if user.role == UserRole::ADMIN { Ok(()) } else { Err(AppError::Forbidden("Forbidden: only admin can access this resource".to_string())) }
}
