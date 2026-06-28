// Deletes the database file and re-runs migrations from scratch
use phoenix_db::connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_path = std::env::var("PHOENIX_DB_PATH")
        .unwrap_or_else(|_| "phoenix.db".to_string());

    println!("WARNING: Resetting database at: {db_path}");

    // Delete the database file
    if std::path::Path::new(&db_path).exists() {
        std::fs::remove_file(&db_path)?;
        // Also remove WAL and SHM files
        let _ = std::fs::remove_file(format!("{db_path}-wal"));
        let _ = std::fs::remove_file(format!("{db_path}-shm"));
        println!("Database deleted.");
    }

    println!("Re-running migrations...");
    let pool = connection::open(&db_path).await?;
    connection::run_migrations(&pool).await?;
    println!("Database reset complete.");
    Ok(())
}
