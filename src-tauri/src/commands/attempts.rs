use crate::db::models::{DayAttempt, UpdateAttemptInput};
use crate::error::Result;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn start_attempt(
    pool: State<'_, SqlitePool>,
    day_plan_id: String,
) -> Result<DayAttempt> {
    let id = Uuid::new_v4().to_string();
    
    let next_attempt_number: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(attempt_number), 0) + 1 FROM day_attempts WHERE day_plan_id = ?"
    )
    .bind(&day_plan_id)
    .fetch_one(pool.inner())
    .await?;
    
    let day_plan_version: i32 = sqlx::query_scalar(
        "SELECT version FROM day_plans WHERE id = ?"
    )
    .bind(&day_plan_id)
    .fetch_one(pool.inner())
    .await?;
    
    let attempt = sqlx::query_as::<_, DayAttempt>(
        "INSERT INTO day_attempts (id, day_plan_id, day_plan_version, attempt_number) 
         VALUES (?, ?, ?, ?) 
         RETURNING *"
    )
    .bind(&id)
    .bind(&day_plan_id)
    .bind(day_plan_version)
    .bind(next_attempt_number)
    .fetch_one(pool.inner())
    .await?;
    
    Ok(attempt)
}

#[tauri::command]
pub async fn get_attempt(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<DayAttempt> {
    let attempt = sqlx::query_as::<_, DayAttempt>(
        "SELECT * FROM day_attempts WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await?
    .ok_or_else(|| crate::error::AppError::NotFound(format!("Attempt {} not found", id)))?;
    
    Ok(attempt)
}

#[tauri::command]
pub async fn update_attempt(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateAttemptInput,
) -> Result<DayAttempt> {
    let mut query = String::from("UPDATE day_attempts SET updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')");
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Sqlite> + Send>> = Vec::new();
    
    if let Some(status) = input.status {
        query.push_str(", status = ?");
        if status == "submitted" {
            query.push_str(", submitted_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), is_draft = 0");
        }
    }
    
    if let Some(score) = input.score_implementation {
        query.push_str(", score_implementation = ?");
    }
    if let Some(score) = input.score_code_quality {
        query.push_str(", score_code_quality = ?");
    }
    if let Some(score) = input.score_accessibility {
        query.push_str(", score_accessibility = ?");
    }
    if let Some(score) = input.score_performance {
        query.push_str(", score_performance = ?");
    }
    if let Some(score) = input.score_quiz {
        query.push_str(", score_quiz = ?");
    }
    if let Some(actual_minutes) = input.actual_minutes {
        query.push_str(", actual_minutes = ?");
    }
    
    query.push_str(" WHERE id = ? RETURNING *");
    
    let attempt = sqlx::query_as::<_, DayAttempt>(&query)
        .bind(&id)
        .fetch_optional(pool.inner())
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound(format!("Attempt {} not found", id)))?;
    
    Ok(attempt)
}

#[tauri::command]
pub async fn list_attempts(
    pool: State<'_, SqlitePool>,
    day_plan_id: String,
) -> Result<Vec<DayAttempt>> {
    let attempts = sqlx::query_as::<_, DayAttempt>(
        "SELECT * FROM day_attempts WHERE day_plan_id = ? ORDER BY attempt_number DESC"
    )
    .bind(&day_plan_id)
    .fetch_all(pool.inner())
    .await?;
    
    Ok(attempts)
}
