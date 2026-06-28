use phoenix_db::connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("phoenix_db=info")
        .init();

    let db_path = std::env::var("PHOENIX_DB_PATH")
        .unwrap_or_else(|_| "phoenix.db".to_string());

    println!("Running migrations on: {db_path}");
    let pool = connection::open(&db_path).await?;
    connection::run_migrations(&pool).await?;
    println!("Migrations complete.");
    Ok(())
}
