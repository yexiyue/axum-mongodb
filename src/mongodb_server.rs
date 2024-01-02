use crate::{servers_warp::ServersWarp, NewWithDb};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
};
use mongodb::Database;
use std::convert::Infallible;

#[derive(Debug, Clone)]
pub struct MongoDbServer<T>
where
    T: Clone,
{
    pub db: Database,
    pub servers: ServersWarp<T>,
}

impl<T> NewWithDb for MongoDbServer<T>
where
    T: Clone + NewWithDb,
{
    fn new(db: &Database) -> Self {
        Self {
            servers: ServersWarp::new(T::new(db)),
            db: db.clone(),
        }
    }
}

impl<T> FromRef<MongoDbServer<T>> for ServersWarp<T>
where
    T: Clone,
{
    fn from_ref(input: &MongoDbServer<T>) -> Self {
        input.servers.clone()
    }
}

#[async_trait]
impl<S, T> FromRequestParts<S> for MongoDbServer<T>
where
    S: Send + Sync,
    Self: FromRef<S>,
    T: Clone,
{
    type Rejection = Infallible;
    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
