此库是使用mongodb为Axum定制的宏，目的是方便使用mongodb数据库

### Example

```rust
#![allow(unused)]

use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
};

use axum::{async_trait, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use axum_mongodb::futures::TryStreamExt;
use axum_mongodb::{preload::*, CollectionInit};
use futures::StreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    options::{ClientOptions, CreateIndexOptions, IndexOptions, ListIndexesOptions},
    Client,
};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Debug, Clone, Column)]
#[dropIndexes]
struct User {
    #[singleIndex(unique)]
    name: String,
    #[compoundIndex(unique, other_fields(name))]
    age: i32,
    #[multikeyIndex(unique, field_name = "age")]
    address: String,
}

#[tokio::main]
#[axum_mongodb::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    let client_options =
        ClientOptions::parse("mongodb://mongodb:password@localhost:21045/admin").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("admin");
    let mongodb_server = MongoDbServer::<Servers>::new(db).await;

    let app = Router::new()
        .route("/", get(hello))
        .route("/db", get(db_test))
        .route("/collection", get(collections_test))
        .route("/collection1", get(collection_test))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR)),
        )
        .with_state(mongodb_server);
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
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

async fn collections_test(user: Servers) -> impl IntoResponse {
    let name = &user.users;
    let indexs = name.list_index_names().await.expect("读取索引失败");
    let name = name.name();
    Json(json!({
        "name":name,
        "indexs":indexs
    }))
}

async fn collection_test(user: Server<User>) -> impl IntoResponse {
    user.drop_indexes(None).await.unwrap_or_else(|e| {});
    (StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable")
}

```

