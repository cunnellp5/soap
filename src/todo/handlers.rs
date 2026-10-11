use crate::todo::{
    state::{Todo},
};
use sqlx::{SqlitePool};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct NewTodo {
    pub(crate) text: String,
}

pub(crate) async fn list_todos(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as::<_, Todo>(
        r#"SELECT id, text, done FROM todos ORDER BY id DESC"#
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn add_todo_db(pool: &SqlitePool, text: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO todos (text, done) VALUES (?1, 0)"#
    )
    .bind(text)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn toggle_todo_db(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    // flip 0/1
    sqlx::query(
        r#"UPDATE todos SET done = CASE done WHEN 0 THEN 1 ELSE 0 END WHERE id = ?1"#
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn delete_todo_db(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"DELETE FROM todos WHERE id = ?1"#
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
