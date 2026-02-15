use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Artifact {
    pub id: String,
    pub day_attempt_id: String,
    pub artifact_type: String,
    pub title: String,
    pub content: String,
    pub file_path: Option<String>,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub url: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArtifactInput {
    pub day_attempt_id: String,
    pub artifact_type: String,
    pub title: String,
    pub content: String,
    pub file_path: Option<String>,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateArtifactInput {
    pub title: Option<String>,
    pub content: Option<String>,
}
