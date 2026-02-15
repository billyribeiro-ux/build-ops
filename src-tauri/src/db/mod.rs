use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use crate::error::Result;
use tauri::Manager;

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
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _schema_migrations (
            name TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
        )",
    )
    .execute(pool)
    .await?;

    let migrations = [
        ("001_create_programs.sql", include_str!("../../migrations/001_create_programs.sql")),
        ("002_create_modules.sql", include_str!("../../migrations/002_create_modules.sql")),
        ("003_create_day_plans.sql", include_str!("../../migrations/003_create_day_plans.sql")),
        (
            "004_create_user_capacity_profiles.sql",
            include_str!("../../migrations/004_create_user_capacity_profiles.sql"),
        ),
        ("005_create_day_attempts.sql", include_str!("../../migrations/005_create_day_attempts.sql")),
        ("006_create_day_sessions.sql", include_str!("../../migrations/006_create_day_sessions.sql")),
        (
            "007_create_time_recommendations.sql",
            include_str!("../../migrations/007_create_time_recommendations.sql"),
        ),
        (
            "008_create_focus_metrics_daily.sql",
            include_str!("../../migrations/008_create_focus_metrics_daily.sql"),
        ),
        (
            "009_create_session_interruptions.sql",
            include_str!("../../migrations/009_create_session_interruptions.sql"),
        ),
        ("010_create_checklists.sql", include_str!("../../migrations/010_create_checklists.sql")),
        ("011_create_quizzes.sql", include_str!("../../migrations/011_create_quizzes.sql")),
        ("012_create_artifacts.sql", include_str!("../../migrations/012_create_artifacts.sql")),
        ("013_create_settings.sql", include_str!("../../migrations/013_create_settings.sql")),
        ("020_create_import_jobs.sql", include_str!("../../migrations/020_create_import_jobs.sql")),
    ];

    let applied_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM _schema_migrations")
        .fetch_one(pool)
        .await?;

    // Legacy bootstrap: if schema already exists but migration tracking does not,
    // mark all migrations as applied to avoid re-running non-idempotent scripts.
    if applied_count == 0 {
        let programs_table_exists: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'programs'",
        )
        .fetch_one(pool)
        .await?;

        if programs_table_exists > 0 {
            for (name, _) in migrations {
                sqlx::query("INSERT OR IGNORE INTO _schema_migrations (name) VALUES (?)")
                    .bind(name)
                    .execute(pool)
                    .await?;
            }
            return Ok(());
        }
    }

    for (name, migration_sql) in migrations {
        let already_applied: Option<i64> =
            sqlx::query_scalar("SELECT 1 FROM _schema_migrations WHERE name = ?")
                .bind(name)
                .fetch_optional(pool)
                .await?;

        if already_applied.is_some() {
            continue;
        }

        sqlx::raw_sql(migration_sql).execute(pool).await?;
        sqlx::query("INSERT INTO _schema_migrations (name) VALUES (?)")
            .bind(name)
            .execute(pool)
            .await?;
    }
    
    Ok(())
}
