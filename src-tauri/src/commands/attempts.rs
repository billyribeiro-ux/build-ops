use crate::db::models::*;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

// Day Attempt Commands (8 commands)

#[tauri::command]
pub async fn start_attempt(
    pool: State<'_, SqlitePool>,
    input: CreateDayAttemptInput,
) -> Result<DayAttempt, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    // Get next attempt number
    let next_attempt_number: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(attempt_number), 0) + 1 FROM day_attempts WHERE day_plan_id = ?"
    )
    .bind(&input.day_plan_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let attempt = sqlx::query_as::<_, DayAttempt>(
        "INSERT INTO day_attempts (
            id, day_plan_id, attempt_number, status, is_draft,
            score_implementation, score_code_quality, score_accessibility,
            score_performance, score_quiz, total_score,
            daily_summary, what_went_well, what_to_improve, key_learnings,
            memory_rebuild_passed, memory_rebuild_notes, actual_minutes,
            started_at, last_autosave, created_at, updated_at
        ) VALUES (?, ?, ?, 'in_progress', 1, 0, 0, 0, 0, 0, 0, '', '', '', '', 0, '', 0, ?, ?, ?, ?)
        RETURNING *"
    )
    .bind(&id)
    .bind(&input.day_plan_id)
    .bind(next_attempt_number)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(attempt)
}

#[tauri::command]
pub async fn get_attempt(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<DayAttempt, String> {
    let attempt = sqlx::query_as::<_, DayAttempt>(
        "SELECT * FROM day_attempts WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(attempt)
}

#[tauri::command]
pub async fn get_current_attempt(
    pool: State<'_, SqlitePool>,
    day_plan_id: String,
) -> Result<Option<DayAttempt>, String> {
    let attempt = sqlx::query_as::<_, DayAttempt>(
        "SELECT * FROM day_attempts 
         WHERE day_plan_id = ? AND is_draft = 1 
         ORDER BY created_at DESC LIMIT 1"
    )
    .bind(&day_plan_id)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(attempt)
}

#[tauri::command]
pub async fn list_attempts(
    pool: State<'_, SqlitePool>,
    day_plan_id: String,
) -> Result<Vec<DayAttemptSummary>, String> {
    let summaries = sqlx::query_as::<_, DayAttemptSummary>(
        "SELECT 
            da.id, da.day_plan_id, dp.title as day_title, dp.day_number,
            da.attempt_number, da.status, da.total_score, da.memory_rebuild_passed,
            da.actual_minutes, da.submitted_at, da.created_at
         FROM day_attempts da
         JOIN day_plans dp ON da.day_plan_id = dp.id
         WHERE da.day_plan_id = ?
         ORDER BY da.attempt_number DESC"
    )
    .bind(&day_plan_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    Ok(summaries)
}

#[tauri::command]
pub async fn update_attempt(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateDayAttemptInput,
) -> Result<DayAttempt, String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    let attempt = sqlx::query_as::<_, DayAttempt>(
        "UPDATE day_attempts SET
            daily_summary = COALESCE(?, daily_summary),
            what_went_well = COALESCE(?, what_went_well),
            what_to_improve = COALESCE(?, what_to_improve),
            key_learnings = COALESCE(?, key_learnings),
            memory_rebuild_passed = COALESCE(?, memory_rebuild_passed),
            memory_rebuild_notes = COALESCE(?, memory_rebuild_notes),
            last_autosave = ?,
            updated_at = ?
         WHERE id = ?
         RETURNING *"
    )
    .bind(input.daily_summary.as_ref())
    .bind(input.what_went_well.as_ref())
    .bind(input.what_to_improve.as_ref())
    .bind(input.key_learnings.as_ref())
    .bind(input.memory_rebuild_passed)
    .bind(input.memory_rebuild_notes.as_ref())
    .bind(&now)
    .bind(&now)
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(attempt)
}

#[tauri::command]
pub async fn autosave_attempt(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query("UPDATE day_attempts SET last_autosave = ?, updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(&now)
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn submit_attempt(
    pool: State<'_, SqlitePool>,
    id: String,
    input: SubmitScoresInput,
) -> Result<DayAttempt, String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    let total_score = input.score_implementation 
        + input.score_code_quality 
        + input.score_accessibility 
        + input.score_performance 
        + input.score_quiz;
    
    // Determine status based on score and memory rebuild
    let status = if !input.memory_rebuild_passed {
        "blocked"
    } else if total_score < 70 {
        "blocked"
    } else if total_score >= 95 {
        "mastery"
    } else {
        "passed"
    };
    
    let attempt = sqlx::query_as::<_, DayAttempt>(
        "UPDATE day_attempts SET
            score_implementation = ?,
            score_code_quality = ?,
            score_accessibility = ?,
            score_performance = ?,
            score_quiz = ?,
            total_score = ?,
            daily_summary = ?,
            what_went_well = ?,
            what_to_improve = ?,
            key_learnings = ?,
            memory_rebuild_passed = ?,
            memory_rebuild_notes = ?,
            status = ?,
            is_draft = 0,
            submitted_at = ?,
            updated_at = ?
         WHERE id = ?
         RETURNING *"
    )
    .bind(input.score_implementation)
    .bind(input.score_code_quality)
    .bind(input.score_accessibility)
    .bind(input.score_performance)
    .bind(input.score_quiz)
    .bind(total_score)
    .bind(&input.daily_summary)
    .bind(&input.what_went_well)
    .bind(&input.what_to_improve)
    .bind(&input.key_learnings)
    .bind(input.memory_rebuild_passed)
    .bind(&input.memory_rebuild_notes)
    .bind(status)
    .bind(&now)
    .bind(&now)
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(attempt)
}

#[tauri::command]
pub async fn delete_attempt(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM day_attempts WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

// Exercise Entry Commands (4 commands)

#[tauri::command]
pub async fn create_exercise_entry(
    pool: State<'_, SqlitePool>,
    input: CreateExerciseEntryInput,
) -> Result<ExerciseEntry, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    let next_order: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(order_index), -1) + 1 FROM exercise_entries WHERE day_attempt_id = ?"
    )
    .bind(&input.day_attempt_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let entry = sqlx::query_as::<_, ExerciseEntry>(
        "INSERT INTO exercise_entries (id, day_attempt_id, language, code, notes, order_index, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)
         RETURNING *"
    )
    .bind(&id)
    .bind(&input.day_attempt_id)
    .bind(&input.language)
    .bind(&input.code)
    .bind(&input.notes)
    .bind(next_order)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(entry)
}

#[tauri::command]
pub async fn update_exercise_entry(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateExerciseEntryInput,
) -> Result<ExerciseEntry, String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    let entry = sqlx::query_as::<_, ExerciseEntry>(
        "UPDATE exercise_entries SET
            language = COALESCE(?, language),
            code = COALESCE(?, code),
            notes = COALESCE(?, notes),
            updated_at = ?
         WHERE id = ?
         RETURNING *"
    )
    .bind(input.language.as_ref())
    .bind(input.code.as_ref())
    .bind(input.notes.as_ref())
    .bind(&now)
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(entry)
}

#[tauri::command]
pub async fn list_exercise_entries(
    pool: State<'_, SqlitePool>,
    day_attempt_id: String,
) -> Result<Vec<ExerciseEntry>, String> {
    let entries = sqlx::query_as::<_, ExerciseEntry>(
        "SELECT * FROM exercise_entries WHERE day_attempt_id = ? ORDER BY order_index"
    )
    .bind(&day_attempt_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    Ok(entries)
}

#[tauri::command]
pub async fn delete_exercise_entry(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM exercise_entries WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

// Artifact Commands (4 commands)

#[tauri::command]
pub async fn create_artifact(
    pool: State<'_, SqlitePool>,
    input: CreateArtifactInput,
) -> Result<Artifact, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    let artifact = sqlx::query_as::<_, Artifact>(
        "INSERT INTO artifacts (
            id, day_attempt_id, artifact_type, title, content,
            file_path, file_size, mime_type, url, created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *"
    )
    .bind(&id)
    .bind(&input.day_attempt_id)
    .bind(&input.artifact_type)
    .bind(&input.title)
    .bind(&input.content)
    .bind(input.file_path.as_ref())
    .bind(input.file_size)
    .bind(input.mime_type.as_ref())
    .bind(input.url.as_ref())
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(artifact)
}

#[tauri::command]
pub async fn update_artifact(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateArtifactInput,
) -> Result<Artifact, String> {
    let artifact = sqlx::query_as::<_, Artifact>(
        "UPDATE artifacts SET
            title = COALESCE(?, title),
            content = COALESCE(?, content)
         WHERE id = ?
         RETURNING *"
    )
    .bind(input.title.as_ref())
    .bind(input.content.as_ref())
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(artifact)
}

#[tauri::command]
pub async fn list_artifacts(
    pool: State<'_, SqlitePool>,
    day_attempt_id: String,
) -> Result<Vec<Artifact>, String> {
    let artifacts = sqlx::query_as::<_, Artifact>(
        "SELECT * FROM artifacts WHERE day_attempt_id = ? ORDER BY created_at DESC"
    )
    .bind(&day_attempt_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    Ok(artifacts)
}

#[tauri::command]
pub async fn delete_artifact(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM artifacts WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

// Bug Log Commands (4 commands)

#[tauri::command]
pub async fn create_bug_log(
    pool: State<'_, SqlitePool>,
    input: CreateBugLogInput,
) -> Result<BugLog, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    let bug_log = sqlx::query_as::<_, BugLog>(
        "INSERT INTO bug_logs (
            id, day_attempt_id, category, severity, symptom,
            root_cause, fix_applied, prevention_strategy,
            time_to_fix_minutes, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *"
    )
    .bind(&id)
    .bind(&input.day_attempt_id)
    .bind(&input.category)
    .bind(&input.severity)
    .bind(&input.symptom)
    .bind(&input.root_cause)
    .bind(&input.fix_applied)
    .bind(&input.prevention_strategy)
    .bind(input.time_to_fix_minutes)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(bug_log)
}

#[tauri::command]
pub async fn update_bug_log(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateBugLogInput,
) -> Result<BugLog, String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    let bug_log = sqlx::query_as::<_, BugLog>(
        "UPDATE bug_logs SET
            category = COALESCE(?, category),
            severity = COALESCE(?, severity),
            symptom = COALESCE(?, symptom),
            root_cause = COALESCE(?, root_cause),
            fix_applied = COALESCE(?, fix_applied),
            prevention_strategy = COALESCE(?, prevention_strategy),
            time_to_fix_minutes = COALESCE(?, time_to_fix_minutes),
            updated_at = ?
         WHERE id = ?
         RETURNING *"
    )
    .bind(input.category.as_ref())
    .bind(input.severity.as_ref())
    .bind(input.symptom.as_ref())
    .bind(input.root_cause.as_ref())
    .bind(input.fix_applied.as_ref())
    .bind(input.prevention_strategy.as_ref())
    .bind(input.time_to_fix_minutes)
    .bind(&now)
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(bug_log)
}

#[tauri::command]
pub async fn list_bug_logs(
    pool: State<'_, SqlitePool>,
    day_attempt_id: String,
) -> Result<Vec<BugLog>, String> {
    let bug_logs = sqlx::query_as::<_, BugLog>(
        "SELECT * FROM bug_logs WHERE day_attempt_id = ? ORDER BY created_at DESC"
    )
    .bind(&day_attempt_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    Ok(bug_logs)
}

#[tauri::command]
pub async fn delete_bug_log(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM bug_logs WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
