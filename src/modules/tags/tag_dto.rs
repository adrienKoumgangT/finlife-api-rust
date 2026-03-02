use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::modules::tags::tag_model::Tag;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TagResponse {
    pub tag_id: Uuid,

    pub name: String,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        Self {
            tag_id: tag.id.unwrap(),
            name: tag.name,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TagCreateRequest {
    pub name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct TagSearchRequest {
    pub name: Option<String>,
}
