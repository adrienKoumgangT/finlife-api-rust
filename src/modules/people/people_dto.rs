use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::people::people_model::People;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeopleResponse {
    pub people_id: Uuid,
    pub user_id: Uuid,
    pub name: String,

    pub email: Option<String>,
    pub phone: Option<String>,
    pub image_url: Option<String>,
    pub note: Option<String>,

    pub archived: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<People> for PeopleResponse {
    fn from(people: People) -> Self {
        Self {
            people_id: people.id.unwrap(),
            user_id: people.user_id,
            name: people.name,
            email: people.email,
            phone: people.phone,
            image_url: people.image_url,
            note: people.note,
            archived: people.archived,
            created_at: people.created_at,
            updated_at: people.updated_at,
        }
    }
}

impl From<&People> for PeopleResponse {
    fn from(people: &People) -> Self {
        Self {
            people_id: people.id.clone().unwrap(),
            user_id: people.user_id.clone(),
            name: people.name.clone(),
            email: people.email.clone(),
            phone: people.phone.clone(),
            image_url: people.image_url.clone(),
            note: people.note.clone(),
            archived: people.archived,
            created_at: people.created_at.clone(),
            updated_at: people.updated_at.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeopleCreateRequest {
    pub name: String,

    pub email: Option<String>,
    pub phone: Option<String>,
    pub image_url: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeopleUpdateRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeopleUpdateArchivedRequest {
    pub archived: bool,
}
