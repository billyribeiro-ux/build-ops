use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Module {
    pub id: String,
    pub program_id: String,
    pub title: String,
    pub description: String,
    pub order_index: i32,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModuleInput {
    pub program_id: String,
    pub title: String,
    pub description: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModuleInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}
