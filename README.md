此库是使用mongodb为Axum定制的宏，目的是方便使用mongodb数据库

### Example

```rust
use axum::{response::IntoResponse, routing::get, Json, Router};
use axum_mongodb::preload::*;
use mongodb::{options::ClientOptions, Client};
use serde_json::json;
use tokio::net::TcpListener;

#[derive(Debug, Clone, Column)]
struct User {
    name: String,
    id: i32,
}

#[tokio::main]
#[axum_mongodb::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_options =
        ClientOptions::parse("mongodb://yexiyue:123456@localhost:27017/test_db").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("test_db");
    let mongodb_server = MongoDbServer::<Servers>::new(&db);

    let app = Router::new()
        .route("/", get(hello))
        .route("/db", get(db_test))
        .route("/collection", get(collection_test))
        .with_state(mongodb_server);
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn hello() -> impl IntoResponse {
    "hello world"
}

#[axum_mongodb::inject]
async fn db_test(servers: DBServers) -> impl IntoResponse {
    let db_name = servers.db.name();
    Json(json!({
        "db_name":db_name
    }))
}

#[axum_mongodb::inject]
async fn collection_test(user: Servers) -> impl IntoResponse {
    let name = &user.users;

    let name = name.name();
    Json(json!({
        "name":name,
    }))
}

```

