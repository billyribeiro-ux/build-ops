use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConceptTag {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub color: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConceptTagInput {
    pub name: String,
    pub domain: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DayPlanTag {
    pub day_plan_id: String,
    pub concept_tag_id: String,
    pub created_at: String,
}
