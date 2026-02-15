use sqlx::SqlitePool;

pub mod models;

pub async fn run_migrations(pool: &SqlitePool) {
    let migrations_dir = std::path::Path::new("src/db/migrations");

    if !migrations_dir.exists() {
        tracing::warn!("Migrations directory not found at {:?}, skipping", migrations_dir);
        return;
    }

    let mut entries: Vec<_> = std::fs::read_dir(migrations_dir)
        .expect("Failed to read migrations directory")
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .extension()
                .map_or(false, |ext| ext == "sql")
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let sql = std::fs::read_to_string(entry.path())
            .unwrap_or_else(|_| panic!("Failed to read migration: {:?}", entry.path()));

        tracing::info!("Running migration: {:?}", entry.file_name());

        // Split by semicolons and execute each statement
        // (SQLx execute doesn't support multi-statement by default for some drivers)
        for statement in sql.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed)
                    .execute(pool)
                    .await
                    .unwrap_or_else(|e| {
                        panic!(
                            "Migration failed: {:?} — Error: {} — Statement: {}",
                            entry.file_name(),
                            e,
                            &trimmed[..trimmed.len().min(100)]
                        );
                    });
            }
        }
    }

    tracing::info!("All migrations completed successfully");
}
