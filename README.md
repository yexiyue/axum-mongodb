# axum-mongodb

[![GitHub Stars](https://img.shields.io/github/stars/yexiyue/axum-mongodb?style=flat-square)](https://github.com/yexiyue/axum-mongodb)
[![Crates.io](https://img.shields.io/crates/v/axum-mongodb?style=flat-square)](https://crates.io/crates/axum-mongodb)

**axum-mongodb** 是一个为 [Axum](https://github.com/tokio-rs/axum) Web 框架量身打造的库，旨在提供一种简洁且优雅的 MongoDB 集成方案。本库的设计灵感来源于著名的 JavaScript 框架 [Nest.js](https://nestjs.com/)，致力于简化并提升 Axum 项目中对 MongoDB 数据库的操作效率。

### 功能亮点
- **基于状态管理的数据库连接**
- **便捷的 CRUD 操作封装**

### 安装

在 `Cargo.toml` 中添加 axum-mongodb 依赖：

```toml
[dependencies]
axum-mongodb = "x.y.z"
```

并通过 `cargo add` 命令快速安装：

```bash
cargo add axum-mongodb
```

### 使用教程

#### 1. 初始化数据库连接

在项目的入口点（如 `lib.rs`）中使用 `axum_mongodb::main` 属性宏来设置 MongoDB 连接和初始化数据库服务。

```rust
use anyhow::Result;
use axum::{response::IntoResponse, routing::get, Router};
use axum_mongodb::preload::*;
use mongodb::{options::ClientOptions, Client};
use tokio::net::TcpListener;

// ...

#[axum_mongodb::main]
pub async fn start() -> Result<()> {
    // 解析并创建 MongoDB 客户端配置
    let client_options = ClientOptions::parse("mongodb://mongodb:password@localhost:21045/admin").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("todo");

    // 创建 MongoDB 服务器状态实例
    let mongodb_server = MongoDbServer::<Servers>::new(db).await;

    // 构建 Axum 应用，并注入 MongoDB 状态到全局路由
    let app = Router::new()
        .route("/", get(hello_world))
        .merge(todos_router())
        .with_state(mongodb_server);

    // 启动服务器监听
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_world() -> impl IntoResponse {
    "hello world"
}
```

#### 2. 定义数据模型

利用 `axum_mongodb::Column` Derive 宏装饰你的结构体以支持与 MongoDB 的交互：

```rust
use anyhow::Result;
use axum_mongodb::futures::TryStreamExt;
use bson::{self, doc, oid::ObjectId};
use mongodb::{results::{DeleteResult, InsertOneResult, UpdateResult},};
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
    update_time: chrono::Local,
}

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

    // ... 其他CRUD方法实现
}
```

#### 3. 在 Axum handler 中使用

在处理函数中注入 `Server<Todo>` 实例，并调用相应的方法完成数据库操作：

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

pub async fn create_todo(todo: Server<Todo>, Json(TodoQuery { description, .. }): Json<TodoQuery>) -> impl IntoResponse {
    let res = todo.create_todo(description).await.unwrap();
    Json(res)
}
```

#### 4. 注册路由

定义并组合相关路由，将 MongoDB 服务状态注入到路由模块中：

```rust
mod controller;
use controller::{create_todo, delete_todo, get_todo, get_todos, update_todo};
use axum::{
    routing::{get, post},
    Router,
};
use axum_mongodb::MongoDbServer;

pub use server::Todo;

pub fn todos_router() -> Router<MongoDbServer<Servers>> {
    Router::new()
        .route("/todos", post(create_todo).get(get_todos))
        .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
}

```

### 示例代码与文档

完整的示例代码可参考 [axum-mongodb-example](https://github.com/yexiyue/axum-mongodb/blob/master/examples/axum/src/lib.rs)。同时，你可以查阅 [API 文档](https://apifox.com/apidoc/shared-6bef1065-5c3e-42a8-bf10-73e21f671fe1) 以获得更详细的信息和示例说明。

### 更多信息

请访问项目主页或查看仓库中的文档以获取更多关于如何在您的 Axum 项目中高效地集成和使用 MongoDB 的细节及高级功能。