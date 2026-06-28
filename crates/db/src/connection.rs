use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::str::FromStr;
use tracing::info;

pub type DbPool = SqlitePool;

/// Open (or create) the SQLite database at `path` with WAL mode and
/// foreign-key enforcement enabled.
pub async fn open(path: &str) -> anyhow::Result<DbPool> {
    let options = SqliteConnectOptions::from_str(&format!("sqlite:{path}?mode=rwc"))?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .foreign_keys(true)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;
    info!(path, "Database connection established");
    Ok(pool)
}

/// Run all pending migrations embedded at compile time.
pub async fn run_migrations(pool: &DbPool) -> anyhow::Result<()> {
    sqlx::migrate!("../../migrations").run(pool).await?;
    info!("Database migrations complete");
    Ok(())
}
