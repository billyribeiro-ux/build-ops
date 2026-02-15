mod error;
mod db;
mod commands;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let pool = db::init_db(&handle).await
                    .expect("Failed to initialize database");
                app.manage(pool);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_program,
            commands::get_program,
            commands::list_programs,
            commands::update_program,
            commands::delete_program,
            commands::create_day_plan,
            commands::get_day_plan,
            commands::list_day_plans,
            commands::start_attempt,
            commands::get_attempt,
            commands::update_attempt,
            commands::list_attempts,
            commands::get_capacity_profile,
            commands::update_capacity_profile,
            commands::create_session,
            commands::start_session,
            commands::pause_session,
            commands::complete_session,
            commands::list_sessions,
            commands::plan_my_day,
            commands::generate_recommendations,
            commands::list_recommendations,
            commands::apply_recommendation,
            commands::dismiss_recommendation,
            commands::get_time_analytics,
            commands::update_daily_metrics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
