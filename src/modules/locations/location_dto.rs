use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::locations::location_model::Location;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationResponse {
    pub location_id: Uuid,

    pub user_id: Uuid,

    pub name: String,

    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,

    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,

    pub archived: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Location> for LocationResponse {
    fn from(location: Location) -> Self {
        Self {
            location_id: location.id.unwrap(),
            user_id: location.user_id,
            name: location.name,
            address: location.address,
            city: location.city,
            district: location.district,
            region: location.region,
            postal_code: location.postal_code,
            country_code: location.country_code,
            latitude: location.latitude,
            longitude: location.longitude,
            archived: location.archived,
            created_at: location.created_at,
            updated_at: location.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationCreateRequest {
    pub name: String,

    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,

    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationUpdateRequest {
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationUpdateLatLongRequest {
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationUpdateArchivedRequest {
    pub archived: bool,
}
