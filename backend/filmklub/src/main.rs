#![allow(unused)]
use tokio;
use axum::Router;
use sqlx::{Sqlite, SqlitePool};
use tower_sessions::{SessionManagerLayer, Expiry};
use tower_sessions_sqlx_store::SqliteStore;
use std::{process::exit, time::Duration};

#[tokio::main]
async fn main()
{
    match dotenvy::dotenv() {
        Ok(_) => {
            println!("Done");
        },
        Err(_) => {
            eprintln!("Error during startup!");
            exit(1)
        },
    }
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL").unwrap()).await.expect("Failed to connect to database!");

    sqlx::migrate!().run(&pool).await.expect("Failed to migrate!");

    let session_store = SqliteStore::new(pool.clone());
    session_store.migrate().await.expect("Failed to migrate session!");

    let session_layer = SessionManagerLayer::new(session_store).with_expiry(Expiry::OnInactivity(tower_sessions::cookie::time::Duration::days(7)));
    
    let app: Router = Router::new()
    //routes
    .layer(session_layer)
    .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
