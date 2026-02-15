use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChecklistItem {
    pub id: String,
    pub day_plan_id: String,
    pub label: String,
    pub is_required: bool,
    pub order_index: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChecklistItemInput {
    pub day_plan_id: String,
    pub label: String,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChecklistItemInput {
    pub label: Option<String>,
    pub is_required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AttemptChecklist {
    pub id: String,
    pub day_attempt_id: String,
    pub checklist_item_id: String,
    pub is_completed: bool,
    pub completed_at: Option<String>,
}
