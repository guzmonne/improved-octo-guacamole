use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub async fn init() -> color_eyre::eyre::Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let migrations_dir = std::env::var("MIGRATIONS_DIR")?;

    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        tracing::info!("Database {database_url} does not exist, creating...");
        match Sqlite::create_database(&database_url).await {
            Ok(_) => {
                tracing::info!("Database {database_url} created.");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to create database {database_url}: {e}");
                Err(color_eyre::Report::new(e))
            }
        }?;
    } else {
        tracing::info!("Database {database_url} exists.");
    }

    tracing::info!("Creating database connection...");
    let db = SqlitePool::connect(&database_url).await?;

    tracing::info!("Running database migrations...");
    let migrations_path = std::path::Path::new(&migrations_dir);
    sqlx::migrate::Migrator::new(migrations_path)
        .await?
        .run(&db)
        .await?;
    tracing::info!("Database migrations complete.");

    Ok(db)
}
