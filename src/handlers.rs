use axum::{
    extract::{Form, Path, State},
    response::Html,
};
use crate::{
    index::render_list,
    state::{AppState, Todo},
};

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct NewTodo {
    text: String,
}

pub(crate) async fn add_todo(
    State(state): State<AppState>,
    Form(new): Form<NewTodo>,
) -> Html<String> {
    let mut todos = state.todos.lock().expect("poisoned");
    let next_id = todos.last().map(|t| t.id + 1).unwrap_or(1);

    todos.push(Todo { 
        id: next_id,
        text: new.text, 
        done: false 
    });

    drop(todos); // release the lock before rendering

    Html(render_list(&state))
}

pub(crate) async fn toggle_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Html<String> {
    let mut todos = state.todos.lock().expect("poisoned");
    if let Some(t) = todos.iter_mut().find(|t| t.id == id) {
        t.done = !t.done;
    }
    drop(todos);
    Html(render_list(&state))
}

pub(crate) async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Html<String> {
    let mut todos = state.todos.lock().expect("poisoned");
    todos.retain(|t| t.id != id);
    drop(todos);
    Html(render_list(&state))
}
