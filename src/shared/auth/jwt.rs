use std::cmp::PartialEq;
use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, HeaderValue},
};
use async_trait::async_trait;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::users::user::user_model::{User, UserRole};
use crate::shared::errors::AppError;
use crate::shared::state::AppState;
use crate::shared::utils::bu;

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
        ).map_err(|_| AppError::Unauthorized) // Or specific "TokenCreationError"
    }

    pub fn verify(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &self.decoding_key,
            &self.validation,
        ).map_err(|_| AppError::Unauthorized)?;

        let c = token_data.claims;
        if c.iss != self.issuer || c.aud != self.audience {
            return Err(AppError::Unauthorized);
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
        Self { user_id: bu(user.id.unwrap().as_slice()), role: user.role }
    }
}

impl From<&User> for AuthUser {
    fn from(user: &User) -> Self {
        Self { user_id: bu(user.id.clone().unwrap().as_slice()), role: user.role.clone() }
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
            .ok_or(AppError::Unauthorized)?;

        let token = bearer_token(auth).ok_or(AppError::Unauthorized)?;

        let claims = state.jwt.verify(token)?;
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;

        Ok(AuthUser { user_id, role: claims.role })
    }
}


pub fn require_admin(user: &AuthUser) -> Result<(), AppError> {
    if user.role == UserRole::ADMIN { Ok(()) } else { Err(AppError::Forbidden) }
}
