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
        ])
        .run(tauri::generate_context!())
        .expect("Error running BuildOps 40");
}
