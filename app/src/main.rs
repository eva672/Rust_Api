use anyhow::Result;
use dotenvy::dotenv;
use std::net::SocketAddr;
use warp::Filter;

mod config;
mod db;
mod error;
mod handlers;
mod jwt;
mod middleware;
mod models;

use config::AppConfig;
use handlers::task_routes;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize logging
    env_logger::init();

    // Load configuration
    let config = AppConfig::from_env()?;

    // Initialize database connection and run migrations
    log::info!("🚀 Starting Rust API application...");
    
    let pool = match db::create_pool().await {
        Ok(pool) => {
            log::info!("✅ Database connection established successfully");
            pool
        }
        Err(e) => {
            log::error!("❌ Failed to connect to database: {}", e);
            log::error!("💡 Make sure PostgreSQL is running and DATABASE_URL is set correctly");
            return Err(e.into());
        }
    };

    // Run database migrations
    if let Err(e) = db::run_migrations(&pool).await {
        log::error!("❌ Database migration failed: {}", e);
        return Err(e.into());
    }

    // Verify database connection
    if let Err(e) = db::verify_connection(&pool).await {
        log::error!("❌ Database verification failed: {}", e);
        return Err(e.into());
    }

    // Populate JWKS cache on startup (optional for now)
    if let Err(e) = jwt::populate_jwks_cache(&config) {
        println!("⚠️  Warning: Could not populate JWKS cache: {}", e);
        println!("   This is expected if Keycloak is not running yet.");
    }

    // Create routes with database pool
    let routes = task_routes(&config, pool).with(
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]),
    );

    // Start server
    let addr: SocketAddr = ([0, 0, 0, 0], config.app_port).into();
    println!(
        "🚀 Server running on http://{}:{}",
        config.app_host, config.app_port
    );

    warp::serve(routes).run(addr).await;

    Ok(())
}
