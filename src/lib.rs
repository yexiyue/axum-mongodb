#[doc(hidden)]
pub use axum::async_trait;

pub use axum_mongodb_core::{inject, main, Column};

#[doc(hidden)]
pub use axum_mongodb_core::inject_meta;
#[doc(hidden)]
pub use futures;
#[doc(hidden)]
pub use mongodb;

mod mongodb_server;
pub use mongodb_server::MongoDbServer;

pub mod preload {
    //! 重新导出常用的结构体和宏

    #[doc(hidden)]
    pub use crate::CollectionInit;
    pub use crate::MongoDbServer;
    #[doc(hidden)]
    pub use crate::NewWithDb;
    pub use axum_mongodb_core::{inject, main, Column};
    pub struct DBServers;
}

#[doc(hidden)]
#[async_trait]
pub trait NewWithDb {
    async fn new(db: mongodb::Database) -> Self;
}

#[doc(hidden)]
#[async_trait]
pub trait CollectionInit {
    async fn init(&self);
}

#[doc(hidden)]
pub struct CreateIndexOptions {
    pub keys: mongodb::bson::Document,
    pub unique: bool,
    pub name: Option<String>,
}
