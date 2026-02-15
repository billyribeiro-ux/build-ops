use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SkillScore {
    pub id: String,
    pub program_id: String,
    pub domain: String,
    pub score: i32,
    pub total_attempts: i32,
    pub last_updated: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRadarData {
    pub domain: String,
    pub score: i32,
    pub max_score: i32,
}
