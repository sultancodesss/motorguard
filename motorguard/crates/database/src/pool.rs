use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tracing::info;

pub type DbPool = SqlitePool;

/// Create and configure the SQLite connection pool.
///
/// For production, swap `SqlitePool` for `PgPool` and update the pool options.
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    info!("Connecting to database: {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(database_url)
        .await?;

    info!("Database pool created successfully");
    Ok(pool)
}

/// Run all pending SQLx migrations.
pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    info!("Running database migrations...");
    sqlx::migrate!("../../migrations").run(pool).await?;
    info!("Migrations completed");
    Ok(())
}
