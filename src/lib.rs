#[doc(hidden)]
pub use axum::async_trait;
pub use axum_mongodb_core::{inject, inject_meta, main, Column};
#[doc(hidden)]
pub use mongodb;

mod mongodb_server;
pub use mongodb_server::MongoDbServer;

pub mod preload {
    pub use crate::CollectionInit;
    pub use crate::MongoDbServer;
    pub use crate::NewWithDb;
    pub use axum_mongodb_core::{inject, inject_meta, main, Column};
    pub struct DBServers;
}

#[async_trait]
pub trait NewWithDb {
    async fn new(db: mongodb::Database) -> Self;
}

#[async_trait]
pub trait CollectionInit {
    async fn init(&self);
}

pub struct CreateIndexOptions {
    pub keys: mongodb::bson::Document,
    pub unique: bool,
    pub name: Option<String>,
}
