mod error;
mod handlers;
mod models;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./dev.db".into());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS items (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            description TEXT
        )",
    )
    .execute(&pool)
    .await?;

    let app = Router::new()
        .route(
            "/items",
            get(handlers::list_items).post(handlers::create_item),
        )
        .route(
            "/items/:id",
            get(handlers::get_item)
                .put(handlers::update_item)
                .delete(handlers::delete_item),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
