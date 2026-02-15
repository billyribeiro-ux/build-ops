#![deny(clippy::all)]
#![warn(clippy::pedantic)]

pub mod db;
pub mod error;
pub mod commands;
// pub mod services;  // Phase 12 - PDF import

use sqlx::sqlite::SqlitePoolOptions;
use tauri::Manager;

pub struct AppState {
    pub db: sqlx::SqlitePool,
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            std::fs::create_dir_all(app_data_dir.join("artifacts"))?;

            let db_path = app_data_dir.join("buildops40.db");
            let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

            tracing_subscriber::fmt()
                .with_env_filter("buildops40=debug,sqlx=warn")
                .init();

            tracing::info!("Database path: {}", db_path.display());

            let pool = tauri::async_runtime::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&db_url)
                    .await
                    .expect("Failed to create database pool");

                // Enable WAL mode for better concurrent read performance
                sqlx::query("PRAGMA journal_mode=WAL")
                    .execute(&pool)
                    .await
                    .expect("Failed to enable WAL mode");

                // Enable foreign keys
                sqlx::query("PRAGMA foreign_keys=ON")
                    .execute(&pool)
                    .await
                    .expect("Failed to enable foreign keys");

                // Run migrations
                db::run_migrations(&pool).await;

                pool
            });

            app.manage(AppState { db: pool });

            tracing::info!("BuildOps 40 initialized successfully");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Phase 2: Program & Module CRUD
            crate::commands::create_program,
            crate::commands::get_program,
            crate::commands::list_programs,
            crate::commands::update_program,
            crate::commands::delete_program,
            crate::commands::duplicate_program,
            crate::commands::get_program_stats,
            crate::commands::create_module,
            crate::commands::get_module,
            crate::commands::list_modules,
            crate::commands::update_module,
            crate::commands::delete_module,
            crate::commands::reorder_modules,
            // Phase 2: Day Plans (19 commands)
            crate::commands::create_day_plan,
            crate::commands::get_day_plan,
            crate::commands::list_day_plans,
            crate::commands::list_day_plans_by_module,
            crate::commands::update_day_plan,
            crate::commands::delete_day_plan,
            crate::commands::reorder_day_plans,
            crate::commands::duplicate_day_plan,
            crate::commands::add_checklist_item,
            crate::commands::update_checklist_item,
            crate::commands::delete_checklist_item,
            crate::commands::reorder_checklist_items,
            crate::commands::add_quiz_question,
            crate::commands::update_quiz_question,
            crate::commands::delete_quiz_question,
            crate::commands::create_concept_tag,
            crate::commands::list_concept_tags,
            crate::commands::add_tag_to_day,
            crate::commands::remove_tag_from_day,
            crate::commands::add_dependency,
            crate::commands::remove_dependency,
            crate::commands::check_dependencies,
            // Phase 3: Attempts & Working Screen (20 commands)
            crate::commands::start_attempt,
            crate::commands::get_attempt,
            crate::commands::get_current_attempt,
            crate::commands::list_attempts,
            crate::commands::update_attempt,
            crate::commands::autosave_attempt,
            crate::commands::submit_attempt,
            crate::commands::delete_attempt,
            crate::commands::create_exercise_entry,
            crate::commands::update_exercise_entry,
            crate::commands::list_exercise_entries,
            crate::commands::delete_exercise_entry,
            crate::commands::create_artifact,
            crate::commands::update_artifact,
            crate::commands::list_artifacts,
            crate::commands::delete_artifact,
            crate::commands::create_bug_log,
            crate::commands::update_bug_log,
            crate::commands::list_bug_logs,
            crate::commands::delete_bug_log,
        ])
        .run(tauri::generate_context!())
        .expect("Error running BuildOps 40");
}
