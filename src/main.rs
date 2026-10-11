mod todo;

use axum::{
    routing::{get, post}, 
    Router
};
use dotenvy::dotenv;
use sqlx::{sqlite::SqlitePoolOptions};
use crate::todo::{
    index::{add_todo, delete_todo, index, toggle_todo},
    state::AppState,
};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // load .env if present

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set, e.g. sqlite://todos.db");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let state = AppState { pool };

    // define a router with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/add", post(add_todo))
        .route("/todos/:id/toggle", axum::routing::patch(toggle_todo))
        .route("/todos/:id", axum::routing::delete(delete_todo))
        .with_state(state);

    // set the address where the server will listen
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    // launch the server
    axum::serve(listener, app)
        .await
        .unwrap();

    Ok(())
}
