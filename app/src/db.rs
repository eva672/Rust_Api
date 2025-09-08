use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

/// Establishes a connection pool to PostgreSQL database
pub async fn create_pool() -> Result<Pool<Postgres>> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable is not set"))?;

    log::info!("🔌 Connecting to PostgreSQL database...");

    // Debug: Always print the database URL (with credentials masked) for debugging
    let masked_url = if let Some(at_pos) = database_url.find('@') {
        if let Some(colon_pos) = database_url[8..].find(':') {
            let start = 8 + colon_pos + 1;
            format!(
                "{}***:***{}",
                &database_url[..start],
                &database_url[at_pos..]
            )
        } else {
            database_url.clone()
        }
    } else {
        database_url.clone()
    };
    log::info!("🔍 Database URL: {}", masked_url);
    log::info!("🔍 Full DATABASE_URL: {}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(std::time::Duration::from_secs(600))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .connect(&database_url)
        .await
        .map_err(|e| {
            log::error!("❌ Failed to connect to database: {}", e);
            anyhow::anyhow!("Database connection failed: {}", e)
        })?;

    // Test the connection
    sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            log::error!("❌ Database connection test failed: {}", e);
            anyhow::anyhow!("Database connection test failed: {}", e)
        })?;

    log::info!("✅ Successfully connected to PostgreSQL database");
    Ok(pool)
}

/// Runs database migrations to create tables if they don't exist
pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<()> {
    log::info!("🔄 Starting database migrations...");

    // Create users table
    log::info!("📋 Creating users table...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            role VARCHAR(50) NOT NULL DEFAULT 'user',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        log::error!("❌ Failed to create users table: {}", e);
        anyhow::anyhow!("Failed to create users table: {}", e)
    })?;
    log::info!("✅ Users table created successfully");

    // Create tasks table
    log::info!("📋 Creating tasks table...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id UUID PRIMARY KEY,
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            title VARCHAR(255) NOT NULL,
            description TEXT,
            completed BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        log::error!("❌ Failed to create tasks table: {}", e);
        anyhow::anyhow!("Failed to create tasks table: {}", e)
    })?;
    log::info!("✅ Tasks table created successfully");

    // Create indexes for better performance
    log::info!("📊 Creating database indexes...");

    let indexes = vec![
        (
            "idx_tasks_user_id",
            "CREATE INDEX IF NOT EXISTS idx_tasks_user_id ON tasks(user_id);",
        ),
        (
            "idx_tasks_completed",
            "CREATE INDEX IF NOT EXISTS idx_tasks_completed ON tasks(completed);",
        ),
        (
            "idx_users_email",
            "CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);",
        ),
        (
            "idx_tasks_created_at",
            "CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at);",
        ),
    ];

    for (name, query) in indexes {
        sqlx::query(query).execute(pool).await.map_err(|e| {
            log::error!("❌ Failed to create index {}: {}", name, e);
            anyhow::anyhow!("Failed to create index {}: {}", name, e)
        })?;
        log::debug!("✅ Index {} created successfully", name);
    }

    log::info!("✅ All database indexes created successfully");
    log::info!("🎉 Database migrations completed successfully");
    Ok(())
}

/// Verifies database connection and returns connection info
pub async fn verify_connection(pool: &Pool<Postgres>) -> Result<()> {
    log::info!("🔍 Verifying database connection...");

    let result: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public'",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        log::error!("❌ Database verification failed: {}", e);
        anyhow::anyhow!("Database verification failed: {}", e)
    })?;

    log::info!(
        "✅ Database connection verified - {} tables found in public schema",
        result.0
    );
    Ok(())
}
