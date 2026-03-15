mod congif;
mod jwt;
mod app_state;

use std::sync::Arc;

use app_state::AppState;
use jwt::JwtKeys;
use sqlx::postgres::PgPoolOptions;
use axum::{Router, routing::get};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::congif::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "auth_server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    let addr = format!("0.0.0.0:{}", config.server_port.clone());
    let _jwt_keys = JwtKeys::load(&config.jwt_private_key_path, &config.jwt_public_key_path, &config.jwt_kid)?;
    tracing::info!("JWT keys loaded");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    let app_state: AppState = AppState { config: Arc::new(config), pool, jwt_keys: Arc::new(_jwt_keys)};
    tracing::info!("Database connected");

    // Router
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .with_state(app_state);

    // Serve
    tracing::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
