## axum-mongodb

本库旨在为Axum项目提供一种更为简洁且优雅的MongoDB集成方案，其设计灵感来源于知名框架[Nest.js](https://nestjs.com/)。通过使用本库，开发者能够在Axum项目中实现对MongoDB数据库的高度简化及高效利用。

### 使用方式

#### 1.安装依赖

```bash
cargo add axum-mongodb
```

#### 2.在入口函数使用main属性宏

**lib.rs**

```rust
use anyhow::Result;
use axum::{response::IntoResponse, routing::get, Router};
use axum_mongodb::preload::*;
use mongodb::{options::ClientOptions, Client};
use tokio::net::TcpListener;
pub mod error;
mod todos;
use todos::Todo;

use crate::todos::todos_router;

// 在lib中使用，这样生成的结构体才能在整个项目中使用
#[axum_mongodb::main]
pub async fn start() -> Result<()> {
    let client_options =
        ClientOptions::parse("mongodb://mongodb:password@localhost:21045/admin").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("todo");

    // 定义State(关键代码)
    let mongodb_server = MongoDbServer::<Servers>::new(db).await;

    let app = Router::new()
        .route("/", get(hello_world))
        .merge(todos_router())
        // 注册State
        .with_state(mongodb_server);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_world() -> impl IntoResponse {
    "hello world"
}
```

**main.rs**

```rust
use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    axum_example::start().await?;
    Ok(())
}
```

#### 3.在结构体上使用Column Derive宏

```rust
use crate::Server;
use anyhow::Result;
use axum_mongodb::futures::TryStreamExt;
// 导入axum-mongodb
use axum_mongodb::preload::*;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Column, Deserialize, Serialize, Clone)]
pub struct Todo {
    #[serde(
        serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string",
        rename = "_id"
    )]
    id: ObjectId,
    description: String,
    completed: bool,
    create_time: chrono::DateTime<chrono::Local>,
    update_time: chrono::DateTime<chrono::Local>,
}

// 实现相应方法，在handler中可以调用到
impl Server<Todo> {
    pub async fn create_todo(&self, description: String) -> Result<InsertOneResult> {
        Ok(self
            .insert_one(
                Todo {
                    id: ObjectId::new(),
                    description,
                    completed: false,
                    create_time: chrono::Local::now(),
                    update_time: chrono::Local::now(),
                },
                None,
            )
            .await?)
    }
		// ...
}

```

#### 4.在axum handler中使用

```rust
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
  	// 关键代码
    todo: Server<Todo>,
    Json(TodoQuery { description, .. }): Json<TodoQuery>,
) -> impl IntoResponse {
    let res = todo.create_todo(description).await.unwrap();
    Json(res)
}

// ...
```

#### 5.注册路由

```rust
mod controller;
use controller::{create_todo, delete_todo, get_todo, get_todos, update_todo};
mod server;
use axum::{
    routing::{get, post},
    Router,
};
use axum_mongodb::MongoDbServer;
pub use server::Todo;

use crate::Servers;

pub fn todos_router() -> Router<MongoDbServer<Servers>> {
    Router::new()
        .route("/todos", post(create_todo).get(get_todos))
        .route(
            "/todos/:id",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
}

```



完整示例代码[axum-mongodb-example](https://github.com/yexiyue/axum-mongodb/blob/master/examples/axum/src/lib.rs)，[示例api文档](https://apifox.com/apidoc/shared-6bef1065-5c3e-42a8-bf10-73e21f671fe1)
