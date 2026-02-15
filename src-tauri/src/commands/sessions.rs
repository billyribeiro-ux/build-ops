use crate::db::models::{DaySession, CreateSessionInput};
use crate::error::Result;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_session(
    pool: State<'_, SqlitePool>,
    input: CreateSessionInput,
) -> Result<DaySession> {
    let id = Uuid::new_v4().to_string();
    
    let session = sqlx::query_as::<_, DaySession>(
        "INSERT INTO day_sessions (id, day_attempt_id, session_type, planned_minutes) 
         VALUES (?, ?, ?, ?) 
         RETURNING *"
    )
    .bind(&id)
    .bind(&input.day_attempt_id)
    .bind(&input.session_type)
    .bind(input.planned_minutes)
    .fetch_one(pool.inner())
    .await?;
    
    Ok(session)
}

#[tauri::command]
pub async fn start_session(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<DaySession> {
    let session = sqlx::query_as::<_, DaySession>(
        "UPDATE day_sessions 
         SET status = 'in_progress', 
             started_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
             updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
         WHERE id = ? 
         RETURNING *"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await?
    .ok_or_else(|| crate::error::AppError::NotFound(format!("Session {} not found", id)))?;
    
    Ok(session)
}

#[tauri::command]
pub async fn pause_session(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<DaySession> {
    let session: DaySession = sqlx::query_as(
        "SELECT * FROM day_sessions WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await?
    .ok_or_else(|| crate::error::AppError::NotFound(format!("Session {} not found", id)))?;
    
    if let Some(started_at) = &session.started_at {
        let started = chrono::DateTime::parse_from_rfc3339(started_at)
            .map_err(|e| crate::error::AppError::Internal(format!("Invalid datetime: {}", e)))?;
        let now = chrono::Utc::now();
        let elapsed_minutes = (now.signed_duration_since(started).num_seconds() / 60) as i32;
        
        let updated_session = sqlx::query_as::<_, DaySession>(
            "UPDATE day_sessions 
             SET actual_minutes = actual_minutes + ?,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
             WHERE id = ? 
             RETURNING *"
        )
        .bind(elapsed_minutes)
        .bind(&id)
        .fetch_one(pool.inner())
        .await?;
        
        Ok(updated_session)
    } else {
        Ok(session)
    }
}

#[tauri::command]
pub async fn complete_session(
    pool: State<'_, SqlitePool>,
    id: String,
    notes: Option<String>,
) -> Result<DaySession> {
    let session: DaySession = sqlx::query_as(
        "SELECT * FROM day_sessions WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await?
    .ok_or_else(|| crate::error::AppError::NotFound(format!("Session {} not found", id)))?;
    
    let mut actual_minutes = session.actual_minutes;
    
    if let Some(started_at) = &session.started_at {
        let started = chrono::DateTime::parse_from_rfc3339(started_at)
            .map_err(|e| crate::error::AppError::Internal(format!("Invalid datetime: {}", e)))?;
        let now = chrono::Utc::now();
        let elapsed_minutes = (now.signed_duration_since(started).num_seconds() / 60) as i32;
        actual_minutes += elapsed_minutes;
    }
    
    let updated_session = sqlx::query_as::<_, DaySession>(
        "UPDATE day_sessions 
         SET status = 'done',
             ended_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
             actual_minutes = ?,
             notes = ?,
             updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
         WHERE id = ? 
         RETURNING *"
    )
    .bind(actual_minutes)
    .bind(notes.unwrap_or_default())
    .bind(&id)
    .fetch_one(pool.inner())
    .await?;
    
    Ok(updated_session)
}

#[tauri::command]
pub async fn list_sessions(
    pool: State<'_, SqlitePool>,
    day_attempt_id: String,
) -> Result<Vec<DaySession>> {
    let sessions = sqlx::query_as::<_, DaySession>(
        "SELECT * FROM day_sessions WHERE day_attempt_id = ? ORDER BY created_at ASC"
    )
    .bind(&day_attempt_id)
    .fetch_all(pool.inner())
    .await?;
    
    Ok(sessions)
}
