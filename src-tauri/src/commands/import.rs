use crate::db::models::import::{GeneratedPlan, ImportJob, ImportJobSummary};
use sqlx::{Pool, Sqlite};
use tauri::AppHandle;
use uuid::Uuid;

#[tauri::command]
pub async fn start_import(
    _app: AppHandle,
    pool: tauri::State<'_, Pool<Sqlite>>,
    file_paths: Vec<String>,
    program_id: Option<String>,
    _api_key: String,
) -> Result<ImportJob, String> {
    let job_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    let source_files_json = serde_json::to_string(&file_paths).unwrap_or_default();
    
    let job = sqlx::query_as::<_, ImportJob>(
        "INSERT INTO import_jobs (
            id, source_type, source_files_json, program_id, status,
            extracted_text, extracted_sections_json, ai_analysis_json,
            generated_plan_json, total_pages, total_tokens, total_days_generated,
            ai_model_used, created_at, updated_at
        ) VALUES (?, 'pdf', ?, ?, 'pending', '', '', '', '', 0, 0, 0, '', ?, ?)
        RETURNING *"
    )
    .bind(&job_id)
    .bind(&source_files_json)
    .bind(program_id)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(job)
}

#[tauri::command]
pub async fn get_import_job(
    pool: tauri::State<'_, Pool<Sqlite>>,
    job_id: String,
) -> Result<ImportJob, String> {
    let job = sqlx::query_as::<_, ImportJob>(
        "SELECT * FROM import_jobs WHERE id = ?"
    )
    .bind(&job_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(job)
}

#[tauri::command]
pub async fn get_import_preview(
    pool: tauri::State<'_, Pool<Sqlite>>,
    job_id: String,
) -> Result<GeneratedPlan, String> {
    let job: ImportJob = sqlx::query_as(
        "SELECT * FROM import_jobs WHERE id = ?"
    )
    .bind(&job_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    if !job.generated_plan_json.is_empty() {
        let plan: GeneratedPlan = serde_json::from_str(&job.generated_plan_json)
            .map_err(|e| format!("Failed to parse plan: {}", e))?;
        Ok(plan)
    } else {
        Err("No generated plan available".to_string())
    }
}

#[tauri::command]
pub async fn update_import_preview(
    pool: tauri::State<'_, Pool<Sqlite>>,
    job_id: String,
    reviewed_plan_json: String,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        "UPDATE import_jobs SET reviewed_plan_json = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&reviewed_plan_json)
    .bind(&now)
    .bind(&job_id)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn apply_import(
    pool: tauri::State<'_, Pool<Sqlite>>,
    job_id: String,
) -> Result<crate::db::models::Program, String> {
    let _job: ImportJob = sqlx::query_as(
        "SELECT * FROM import_jobs WHERE id = ?"
    )
    .bind(&job_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    // TODO: Implement actual import application logic
    Err("Import application not yet implemented".to_string())
}

#[tauri::command]
pub async fn cancel_import(
    pool: tauri::State<'_, Pool<Sqlite>>,
    job_id: String,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        "UPDATE import_jobs SET status = 'cancelled', updated_at = ? WHERE id = ?"
    )
    .bind(&now)
    .bind(&job_id)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn list_import_jobs(
    pool: tauri::State<'_, Pool<Sqlite>>,
) -> Result<Vec<ImportJobSummary>, String> {
    let jobs: Vec<ImportJobSummary> = sqlx::query_as(
        "SELECT id, source_type, status, total_days_generated, created_at, updated_at 
         FROM import_jobs 
         ORDER BY created_at DESC"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(jobs)
}

#[tauri::command]
pub async fn delete_import_job(
    pool: tauri::State<'_, Pool<Sqlite>>,
    job_id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM import_jobs WHERE id = ?")
        .bind(&job_id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
