use axum::{extract::Path, response::IntoResponse, Json};
use serde::Deserialize;

use super::Todo;
use crate::Server;

#[derive(Debug, Deserialize)]
pub struct TodoQuery {
    pub description: String,
    pub completed: Option<bool>,
}

pub async fn create_todo(
    todo: Server<Todo>,
    Json(TodoQuery { description, .. }): Json<TodoQuery>,
) -> impl IntoResponse {
    let res = todo.create_todo(description).await.unwrap();
    Json(res)
}

pub async fn get_todos(todo: Server<Todo>) -> impl IntoResponse {
    let res = todo.get_todos().await.unwrap();
    Json(res)
}

pub async fn get_todo(todo: Server<Todo>, Path(id): Path<String>) -> impl IntoResponse {
    tracing::info!("get_todo: {}", id);
    let res = todo.get_todo(id).await.unwrap();
    Json(res)
}

pub async fn delete_todo(todo: Server<Todo>, Path(id): Path<String>) -> impl IntoResponse {
    let res = todo.delete_todo(id).await.unwrap();
    Json(res)
}

pub async fn update_todo(
    todo: Server<Todo>,
    Path(id): Path<String>,
    Json(TodoQuery {
        description,
        completed,
    }): Json<TodoQuery>,
) -> impl IntoResponse {
    let res = todo.update_todo(id, description, completed).await.unwrap();
    Json(res)
}
