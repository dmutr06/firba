use anyhow::Result;
use axum::Router;
use firba::{auth, init_db, todos, users, AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    pretty_env_logger::init();

    log::info!("Connecting to db...");
    init_db(&std::env::var("DATABASE_URL")?).await;

    log::info!("Setting up an app...");
    let app = Router::new()
        .nest("/users", users::router())
        .nest("/auth", auth::router())
        .nest("/todos", todos::router())
        .with_state(AppState);

    log::info!("Starting a server...");
    let listener = TcpListener::bind("localhost:6969").await.unwrap();

    log::info!("Server has been started");
    axum::serve(listener, app).await?;

    Ok(())
}
