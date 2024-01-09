mod controller;
use controller::{create_todo, delete_todo, get_todo, get_todos, update_todo};
mod server;
use axum::{
    routing::{get, post},
    Router,
};
pub use server::Todo;

pub fn todos_router() -> Router {
    Router::new()
        .route("/todos", post(create_todo).get(get_todos))
        .route(
            "/todos/:id",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
}
