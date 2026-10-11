use sqlx::{FromRow, SqlitePool};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) pool: SqlitePool,
}

#[derive(FromRow, Debug, Clone)]
pub(crate) struct Todo {
    pub(crate) id: i64,
    pub(crate) text: String,
    pub(crate) done: i64, // 0 or 1 in SQLite
    // created_at can be added if you need it in UI:
    // created_at: chrono::NaiveDateTime,
}
