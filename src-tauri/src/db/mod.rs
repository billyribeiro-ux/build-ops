use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::PathBuf;
use crate::error::Result;

pub mod models;

pub async fn init_db(app_handle: &tauri::AppHandle) -> Result<SqlitePool> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| crate::error::AppError::Internal(format!("Failed to get app dir: {}", e)))?;
    
    std::fs::create_dir_all(&app_dir)?;
    
    let db_path = app_dir.join("buildops40.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    
    run_migrations(&pool).await?;
    
    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    let migrations = vec![
        include_str!("../migrations/001_create_programs.sql"),
        include_str!("../migrations/002_create_modules.sql"),
        include_str!("../migrations/003_create_day_plans.sql"),
        include_str!("../migrations/004_create_user_capacity_profiles.sql"),
        include_str!("../migrations/005_create_day_attempts.sql"),
        include_str!("../migrations/006_create_day_sessions.sql"),
        include_str!("../migrations/007_create_time_recommendations.sql"),
        include_str!("../migrations/008_create_focus_metrics_daily.sql"),
        include_str!("../migrations/009_create_session_interruptions.sql"),
        include_str!("../migrations/010_create_checklists.sql"),
        include_str!("../migrations/011_create_quizzes.sql"),
        include_str!("../migrations/012_create_artifacts.sql"),
        include_str!("../migrations/013_create_settings.sql"),
        include_str!("../migrations/020_create_import_jobs.sql"),
    ];
    
    for migration in migrations {
        sqlx::raw_sql(migration).execute(pool).await?;
    }
    
    Ok(())
}
