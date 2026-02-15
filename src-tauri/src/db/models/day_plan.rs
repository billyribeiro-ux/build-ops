use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{ChecklistItem, QuizQuestion, ConceptTag, DayDependency};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DayPlan {
    pub id: String,
    pub program_id: String,
    pub module_id: String,
    pub title: String,
    pub day_number: i32,
    pub version: i32,
    pub status: String,
    pub syntax_targets: String,
    pub implementation_brief: String,
    pub files_to_create: String,
    pub success_criteria: String,
    pub stretch_challenge: String,
    pub notes: String,
    pub estimated_minutes: i32,
    pub memory_rebuild_minutes: i32,
    pub min_minutes: i32,
    pub recommended_minutes: i32,
    pub deep_minutes: i32,
    pub complexity_level: i32,
    pub focus_blocks: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayPlanFull {
    #[serde(flatten)]
    pub day_plan: DayPlan,
    pub checklist_items: Vec<ChecklistItem>,
    pub quiz_questions: Vec<QuizQuestion>,
    pub concept_tags: Vec<ConceptTag>,
    pub dependencies: Vec<DayDependency>,
    pub module_title: String,
    pub module_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DayPlanSummary {
    pub id: String,
    pub title: String,
    pub day_number: i32,
    pub module_id: String,
    pub module_title: String,
    pub module_color: String,
    pub status: String,
    pub estimated_minutes: i32,
    pub checklist_count: i32,
    pub quiz_count: i32,
    pub tag_count: i32,
    pub best_score: Option<i32>,
    pub attempt_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusBlock {
    pub session_type: String,
    pub minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDayPlanInput {
    pub program_id: String,
    pub module_id: String,
    pub title: String,
    pub day_number: i32,
    pub syntax_targets: String,
    pub implementation_brief: String,
    pub min_minutes: Option<i32>,
    pub recommended_minutes: Option<i32>,
    pub deep_minutes: Option<i32>,
    pub complexity_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDayPlanInput {
    pub title: Option<String>,
    pub syntax_targets: Option<String>,
    pub implementation_brief: Option<String>,
    pub files_to_create: Option<String>,
    pub success_criteria: Option<String>,
    pub stretch_challenge: Option<String>,
    pub notes: Option<String>,
    pub status: Option<String>,
    pub estimated_minutes: Option<i32>,
    pub memory_rebuild_minutes: Option<i32>,
}
