use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuizQuestion {
    pub id: String,
    pub day_plan_id: String,
    pub question_text: String,
    pub question_type: String,
    pub correct_answer: String,
    pub options: String,
    pub points: i32,
    pub time_limit_seconds: i32,
    pub order_index: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuizQuestionInput {
    pub day_plan_id: String,
    pub question_text: String,
    pub question_type: String,
    pub correct_answer: String,
    pub options: Option<String>,
    pub points: i32,
    pub time_limit_seconds: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateQuizQuestionInput {
    pub question_text: Option<String>,
    pub question_type: Option<String>,
    pub correct_answer: Option<String>,
    pub options: Option<String>,
    pub points: Option<i32>,
    pub time_limit_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuizAttempt {
    pub id: String,
    pub day_attempt_id: String,
    pub quiz_question_id: String,
    pub answer: String,
    pub is_correct: bool,
    pub time_taken_seconds: i32,
    pub submitted_at: String,
}
