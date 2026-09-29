mod handlers;
mod index;
mod state;

use axum::{routing::{get, post}, Router};
use handlers::{add_todo, delete_todo, toggle_todo};
use index::index;
use state::AppState;

use std::{sync::{Arc, Mutex}, net::SocketAddr};

#[tokio::main]
async fn main() {
    let state = AppState{
        todos: Arc::new(Mutex::new(Vec::new()))
    };

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
}