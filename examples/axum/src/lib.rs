use anyhow::Result;
use axum::{response::IntoResponse, routing::get, Extension, Router};
use axum_mongodb::preload::*;
use mongodb::{options::ClientOptions, Client};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
pub mod error;
mod todos;
use todos::Todo;

use crate::todos::todos_router;

// 在lib中使用，这样生成的结构体才能在整个项目中使用
#[axum_mongodb::main]
pub async fn start() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // 连接数据库
    let client_options =
        ClientOptions::parse("mongodb://mongodb:password@localhost:21045/admin").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("todo");

    // 定义State
    let mongodb_server = MongoDbServer::<Servers>::new(db).await;

    let app = Router::new()
        .route("/", get(hello_world))
        .merge(todos_router())
        // 启用日志
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR)),
        )
        // 注册State
        .layer(Extension(mongodb_server));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_world() -> impl IntoResponse {
    "hello world"
}
