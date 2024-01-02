pub use axum_mongodb_core::{inject, inject_meta, main, Column};
pub use futures;
pub use mongodb;
mod mongodb_server;
mod server;
mod servers_warp;
pub use mongodb_server::MongoDbServer;
pub use server::Server;
pub use servers_warp::ServersWarp;

pub mod preload {
    pub use crate::MongoDbServer;
    pub use crate::NewWithDb;
    pub use crate::Server;
    pub use crate::ServersWarp;
    pub use axum_mongodb_core::{inject, inject_meta, main, Column};
    pub struct Servers;
    pub struct DBServers;
}

pub trait NewWithDb {
    fn new(db: &mongodb::Database) -> Self;
}
