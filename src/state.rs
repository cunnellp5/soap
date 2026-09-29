use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub(crate) todos: Arc<Mutex<Vec<Todo>>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Todo {
    pub(crate) id: u64,
    pub(crate) text: String,
    pub(crate) done: bool,
}