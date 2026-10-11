use axum::{extract::{Form, Path, State}, response::Html};
use crate::todo::handlers::{add_todo_db, delete_todo_db, list_todos, toggle_todo_db, NewTodo};
use crate::todo::state::AppState;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    let list_html = render_list(&state).await.unwrap_or_else(|e| {
        format!(r#"<div id="todo-list"><ul>
            <li class="muted">Error loading todos: {}</li>
        </ul></div>"#, html_escape(&e.to_string()))
    });

    Html(format!(
        r##"<!doctype html>
        <html lang="en">
            <head>
              <meta charset="utf-8">
              <meta name="viewport" content="width=device-width,initial-scale=1">
              <title>Rust Axum HTMX Todo App</title>
              <script src="https://unpkg.com/htmx.org@1.9.10"></script>
              <style>
                body {{ font-family: system-ui, -apple-system, segoe ui, roboto, sans-serif; margin: 2rem; }}
                h1 {{ margin-bottom: 1rem; }}
                form {{ display: flex; gap: .5rem; margin-bottom: 1rem; }}
                input[type=text] {{ flex: 1; padding: .6rem .8rem; border: 1px solid #ddd; border-radius: .5rem; }}
                button {{ padding: .6rem 1rem; border: 0; border-radius: .5rem; background: #f97316; color: #fff; cursor: pointer; }}
                ul {{ list-style: none; padding: 0; }}
                li {{ display: flex; align-items: center; gap: .6rem; padding: .4rem 0; border-bottom: 1px dashed #eee; }}
                .done {{ text-decoration: line-through; color: #888; }}
                .muted {{ color: #666; font-size: .9rem; }}
                li button {{ margin-left: auto; background:#ef4444; color:#fff; border-radius:.4rem; padding:.3rem .6rem; }}
                li input[type=checkbox] {{ transform: translateY(1px); }}
              </style>
            </head>
            <body>
              <h1>Todos</h1>
            
              <form hx-post="/add" hx-target="#todo-list" hx-swap="outerHTML"
                    hx-on::after-request="this.reset(); this.querySelector('input[name=text]').focus();">
                <input type="text" name="text" placeholder="Add a task…" required autofocus>
                <button type="submit">Add</button>
              </form>
            
              {list_html}
            
              <p class="muted">Data is persisted with SQLite using sqlx.</p>
            </body>
        </html>"##
    ))
}


pub(crate) async fn add_todo(
    State(state): State<AppState>,
    Form(new): Form<NewTodo>,
) -> Html<String> {
    let _ = add_todo_db(&state.pool, &new.text).await;
    Html(render_list(&state).await.unwrap_or_else(error_list))
}

pub(crate) async fn toggle_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Html<String> {
    let _ = toggle_todo_db(&state.pool, id).await;
    Html(render_list(&state).await.unwrap_or_else(error_list))
}

pub(crate) async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Html<String> {
    let _ = delete_todo_db(&state.pool, id).await;
    Html(render_list(&state).await.unwrap_or_else(error_list))
}

pub(crate) async fn render_list(state: &AppState) -> Result<String, sqlx::Error> {
    let todos = list_todos(&state.pool).await?;
    let items = if todos.is_empty() {
        r#"<li class="muted">No tasks yet. Add your first one above.</li>"#.to_string()
    } else {
        todos
            .iter()
            .map(|t| {
                let cls = if t.done == 1 { "done" } else { "" };
                let checked = if t.done == 1 { "checked" } else { "" };
                format!(
r##"<li>
  <input type="checkbox" {checked}
         hx-patch="/todos/{id}/toggle"
         hx-target="#todo-list" hx-swap="outerHTML">
  <span class="{cls}">{text}</span>
  <button
      hx-delete="/todos/{id}"
      hx-target="#todo-list" hx-swap="outerHTML"
      aria-label="Delete">✕</button>
</li>"##,
                    id = t.id,
                    text = html_escape(&t.text),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    Ok(format!(r#"<div id="todo-list"><ul>{items}</ul></div>"#))
}

fn error_list<E: std::fmt::Display>(e: E) -> String {
    format!(r#"<div id="todo-list"><ul>
        <li class="muted">Error: {}</li>
    </ul></div>"#, html_escape(&e.to_string()))
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
